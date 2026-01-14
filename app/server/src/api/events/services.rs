use super::endpoints;
use crate::{
    api::events::{Event, EventError, EventFilters, NewEvent},
    state::GlobalState,
    utils::event_encryption::decrypt_json,
    utils::redaction::{redact_json_value, redact_string},
};
use axum::{Router, middleware::from_fn_with_state, routing::get};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
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

pub fn log_event_sync(state: &GlobalState, new_event: &NewEvent) -> Result<Event, EventError> {
    use crate::schema::events;

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(events::table)
        .values(new_event)
        .get_result::<Event>(&mut conn)
        .map_err(EventError::from)
}

pub fn query_events(state: &GlobalState, filters: EventFilters) -> Result<Vec<Event>, EventError> {
    use crate::schema::events::dsl::{events, user_id, resource_type, resource_id, action, created_at};

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
            if let Some(ciphertext) = encrypted_changes.as_str() {
                match decrypt_json(&state.event_encryption_key, ciphertext, nonce) {
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

    if !is_admin
        && let Some(uid) = &event.user_id {
            event.user_id = Some(redact_string(uid));
        }

    event
}
