use std::collections::HashMap;

use super::endpoints;
use crate::{
    api::events::{
        AuditLogFilters, Event, EventError, EventFilters, EventLogResponse, EventLogUser, NewEvent,
        PaginatedEventLogResponse,
    },
    state::GlobalState,
    utils::event_encryption::{decrypt_json, encrypt_json},
    utils::redaction::{redact_json_value, redact_string},
};
use axum::{Router, middleware::from_fn_with_state, routing::get};
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
};
use tracing::error;

pub fn get_routes(state: &GlobalState) -> Router {
    Router::new()
        .route("/", get(endpoints::list_events))
        .route("/{id}", get(endpoints::get_event))
        .layer(from_fn_with_state(
            state.clone(),
            crate::middlewares::auth::middleware,
        ))
        .with_state(state.clone())
}

/// Encrypt a metadata JSON value with the server's event encryption key.
/// Returns a tuple `(ciphertext_value, nonce)` ready to pass into
/// `NewEvent::with_metadata` / `with_metadata_nonce`. On failure, logs the
/// error and returns the plaintext value with `None` nonce so the event can
/// still be persisted (degraded mode).
#[must_use]
pub fn encrypt_metadata(
    state: &GlobalState,
    value: serde_json::Value,
) -> (serde_json::Value, Option<Vec<u8>>) {
    match encrypt_json(&state.event_encryption_key, &value) {
        Ok((ciphertext, nonce)) => (serde_json::json!(ciphertext), Some(nonce)),
        Err(e) => {
            error!("Failed to encrypt event metadata: {e}");
            (value, None)
        }
    }
}

pub fn log_event_sync(state: &GlobalState, new_event: &NewEvent) -> Result<Event, EventError> {
    use crate::schema::events;

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(events::table)
        .values(new_event)
        .returning(Event::as_select())
        .get_result::<Event>(&mut conn)
        .map_err(EventError::from)
}

pub fn query_events(state: &GlobalState, filters: EventFilters) -> Result<Vec<Event>, EventError> {
    use crate::schema::events::dsl::{
        action, created_at, events, resource_id, resource_type, user_id,
    };

    let mut conn = state.get_db_conn()?;
    let mut query = events.into_boxed();

    if let Some(uid) = filters.user_id {
        query = query.filter(user_id.eq(uid));
    }

    if let Some(rt) = filters.resource_type {
        query = query.filter(resource_type.eq(rt));
    }

    if let Some(rid) = filters.resource_id {
        query = query.filter(resource_id.eq(rid));
    }

    if let Some(act) = filters.action {
        query = query.filter(action.eq(act));
    }

    if let Some(from) = filters.date_from {
        query = query.filter(created_at.ge(from));
    }

    if let Some(to) = filters.date_to {
        query = query.filter(created_at.le(to));
    }

    query = query.order(created_at.desc());

    let limit_val = filters.limit.unwrap_or(100).min(1000);
    query = query.limit(limit_val);

    if let Some(offset_val) = filters.offset {
        query = query.offset(offset_val);
    }

    query
        .select(Event::as_select())
        .load::<Event>(&mut conn)
        .map_err(EventError::from)
}

/// Query shell history events (`AgentCommand` + `AgentShellOutput`) for a
/// specific agent under an audit, ordered chronologically ascending for replay.
pub fn query_shell_history(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    limit: i64,
) -> Result<Vec<Event>, EventError> {
    use crate::api::events::EventAction;
    use crate::schema::events::dsl::{action, created_at, events, resource_id};

    let rid = format!("agent_id:{agent_id},audit_id:{audit_id}");
    let clamped = limit.clamp(1, 2000);

    let mut conn = state.get_db_conn()?;

    events
        .filter(resource_id.eq(rid))
        .filter(action.eq_any([EventAction::AgentCommand, EventAction::AgentShellOutput]))
        .order(created_at.asc())
        .limit(clamped)
        .select(Event::as_select())
        .load::<Event>(&mut conn)
        .map_err(EventError::from)
}

pub fn get_event_by_id(state: &GlobalState, event_id: &str) -> Result<Option<Event>, EventError> {
    use crate::schema::events::dsl::{events, id};

    let mut conn = state.get_db_conn()?;

    events
        .filter(id.eq(event_id))
        .select(Event::as_select())
        .first::<Event>(&mut conn)
        .optional()
        .map_err(EventError::from)
}

pub fn decrypt_and_redact_event(state: &GlobalState, mut event: Event, is_admin: bool) -> Event {
    if let Some(encrypted_metadata) = &event.metadata {
        if let Some(nonce) = &event.metadata_nonce {
            if let Some(ciphertext) = encrypted_metadata.as_str() {
                match decrypt_json(&state.event_encryption_key, ciphertext, nonce) {
                    Ok(mut decrypted) => {
                        redact_json_value(&mut decrypted, is_admin);
                        event.metadata = Some(decrypted);
                    }
                    Err(e) => {
                        error!("Failed to decrypt metadata: {}", e);
                        event.metadata = Some(serde_json::json!({"error": "decryption_failed"}));
                    }
                }
            }
        } else if let Some(mut metadata) = event.metadata.clone() {
            redact_json_value(&mut metadata, is_admin);
            event.metadata = Some(metadata);
        }
    }

    if let Some(encrypted_changes) = &event.changes {
        if let Some(nonce) = &event.changes_nonce {
            // Changes may be stored as a plain string or as {before, after} with encrypted values
            let ciphertext = encrypted_changes
                .as_str()
                .map(String::from)
                .or_else(|| {
                    encrypted_changes
                        .get("after")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                });

            if let Some(ct) = ciphertext {
                match decrypt_json(&state.event_encryption_key, &ct, nonce) {
                    Ok(mut decrypted) => {
                        redact_json_value(&mut decrypted, is_admin);
                        event.changes = Some(decrypted);
                    }
                    Err(e) => {
                        error!("Failed to decrypt changes: {}", e);
                        event.changes = Some(serde_json::json!({"error": "decryption_failed"}));
                    }
                }
            }
        } else if let Some(mut changes) = event.changes.clone() {
            redact_json_value(&mut changes, is_admin);
            event.changes = Some(changes);
        }
    }

    if !is_admin && let Some(uid) = &event.user_id {
        event.user_id = Some(redact_string(uid));
    }

    event
}

pub fn query_audit_logs(
    state: &GlobalState,
    audit_id: &str,
    filters: &AuditLogFilters,
) -> Result<PaginatedEventLogResponse, EventError> {
    use crate::schema::events::dsl::{action, created_at, events, request_path};
    use crate::schema::users;

    let mut conn = state.get_db_conn()?;
    let path_prefix = format!("/v1/audits/{audit_id}");

    // Build base query for counting
    let mut count_query = events.into_boxed();
    count_query = count_query.filter(request_path.like(format!("{path_prefix}%")));

    if let Some(ref act) = filters.action {
        count_query = count_query.filter(action.eq(act));
    }

    let total = count_query
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(EventError::from)?;

    // Build data query
    let mut data_query = events.into_boxed();
    data_query = data_query.filter(request_path.like(format!("{path_prefix}%")));

    if let Some(ref act) = filters.action {
        data_query = data_query.filter(action.eq(act));
    }

    data_query = data_query.order(created_at.desc());

    let limit_val = filters.limit.unwrap_or(50).min(200);
    let offset_val = filters.offset.unwrap_or(0);
    data_query = data_query.limit(limit_val).offset(offset_val);

    let rows = data_query
        .select(Event::as_select())
        .load::<Event>(&mut conn)
        .map_err(EventError::from)?;

    let include_user = filters.include_user.unwrap_or(false);

    let users_map: HashMap<String, crate::api::users::User> = if include_user {
        let user_ids: Vec<String> = rows.iter().filter_map(|e| e.user_id.clone()).collect();
        if user_ids.is_empty() {
            HashMap::new()
        } else {
            users::table
                .filter(users::id.eq_any(&user_ids))
                .select(crate::api::users::User::as_select())
                .load::<crate::api::users::User>(&mut conn)
                .map_err(EventError::from)?
                .into_iter()
                .map(|u| (u.id.clone(), u))
                .collect()
        }
    } else {
        HashMap::new()
    };

    let data = rows
        .into_iter()
        .map(|event| {
            // Resolve user BEFORE decrypting (decrypt_and_redact redacts user_id)
            let user = event
                .user_id
                .as_ref()
                .and_then(|uid| users_map.get(uid))
                .map(|u| EventLogUser {
                    id: u.id.clone(),
                    username: u.username.clone(),
                    email: u.email.clone(),
                    first_name: u.first_name.clone(),
                    last_name: u.last_name.clone(),
                    avatar_url: crate::api::users::map_avatar_url(u.avatar_url.clone()),
                });

            // Decrypt metadata/changes after user lookup
            let decrypted = decrypt_and_redact_event(state, event, true);

            EventLogResponse {
                id: decrypted.id,
                user_id: decrypted.user_id,
                user,
                action: decrypted.action,
                resource_type: decrypted.resource_type,
                resource_id: decrypted.resource_id,
                request_method: decrypted.request_method,
                request_path: decrypted.request_path,
                metadata: decrypted.metadata,
                changes: decrypted.changes,
                created_at: decrypted.created_at.to_string(),
            }
        })
        .collect();

    Ok(PaginatedEventLogResponse {
        data,
        total,
        limit: limit_val,
        offset: offset_val,
    })
}
