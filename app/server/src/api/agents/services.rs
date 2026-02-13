use super::{AgentError, AgentResponse, AgentTokenResponse, AuditAgent, CreateAgentPayload};
use crate::{
    api::audits::{self, AuditError},
    middlewares::auth,
    state::GlobalState,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, post},
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use sha2::{Digest, Sha256};

use super::endpoints;

/// Get routes for agents module (nested under /audits)
pub fn get_routes() -> Router<GlobalState> {
    Router::new()
        .route(
            "/{audit_id}/agents",
            post(endpoints::create_agent).get(endpoints::list_agents),
        )
        .route(
            "/{audit_id}/agents/{agent_id}",
            delete(endpoints::revoke_agent),
        )
        .layer(from_fn(auth::middleware))
}

/// Generate a cryptographically random token
fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: [u8; 32] = rng.random();
    bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        use std::fmt::Write;
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Hash a token with SHA-256
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Create a new agent for an audit
///
/// # Errors
/// Returns `AgentError` if validation fails or database error
pub fn create_agent(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    payload: CreateAgentPayload,
    is_admin: bool,
) -> Result<AgentTokenResponse, AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    // Check name uniqueness within audit
    let mut conn = state.get_db_conn()?;
    let existing = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .filter(audit_agents::name.eq(&payload.name))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?;

    if existing.is_some() {
        return Err(AgentError::NameAlreadyExists);
    }

    let raw_token = generate_token();
    let token_hash = hash_token(&raw_token);

    let agent = AuditAgent::new(audit_id.to_string(), payload.name, token_hash);

    diesel::insert_into(audit_agents::table)
        .values(&agent)
        .execute(&mut conn)?;

    Ok(AgentTokenResponse {
        id: agent.id,
        audit_id: agent.audit_id,
        name: agent.name,
        token: raw_token,
    })
}

/// List all agents for an audit
///
/// # Errors
/// Returns `AgentError` if database error
pub fn list_agents(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<Vec<AgentResponse>, AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agents = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .load::<AuditAgent>(&mut conn)?;

    let registry = state.agent_registry.lock().ok();
    let responses = agents
        .into_iter()
        .map(|agent| {
            let (is_online, last_hb, sys_info) = match &registry {
                Some(reg) => match reg.get(&agent.id) {
                    Some(handle) => {
                        let online = handle.is_online(30);
                        let hb = handle
                            .last_heartbeat
                            .lock()
                            .ok()
                            .and_then(|hb| *hb)
                            .map(|ts| ts.to_rfc3339());
                        let si = handle
                            .system_info
                            .lock()
                            .ok()
                            .and_then(|si| si.clone())
                            .map(super::SystemInfoResponse::from);
                        (online, hb, si)
                    }
                    None => (false, None, None),
                },
                None => (false, None, None),
            };
            AgentResponse {
                id: agent.id,
                audit_id: agent.audit_id,
                name: agent.name,
                is_revoked: agent.is_revoked,
                is_online,
                last_connected_at: agent.last_connected_at.map(|d| d.to_string()),
                last_heartbeat: last_hb,
                system_info: sys_info,
                created_at: agent.created_at.to_string(),
            }
        })
        .collect();

    Ok(responses)
}

/// Revoke an agent
///
/// # Errors
/// Returns `AgentError` if agent not found or database error
pub fn revoke_agent(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    diesel::update(audit_agents::table.filter(audit_agents::id.eq(agent_id)))
        .set(audit_agents::is_revoked.eq(true))
        .execute(&mut conn)?;

    // Disconnect the agent if currently connected
    if let Ok(registry) = state.agent_registry.lock()
        && let Some(handle) = registry.get(agent_id)
    {
        handle.shutdown();
    }

    Ok(())
}

/// Verify an agent token against the database.
/// Returns the agent ID if valid.
///
/// # Errors
/// Returns `AgentError` if token invalid, agent revoked, or database error
pub fn verify_agent_token(
    state: &GlobalState,
    audit_id: &str,
    token: &str,
) -> Result<AuditAgent, AgentError> {
    use crate::schema::audit_agents;

    let token_hash = hash_token(token);
    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .filter(audit_agents::token_hash.eq(&token_hash))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::InvalidToken)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    // Check audit still exists and is not archived
    let audit = audits::get_audit(state, audit_id).map_err(|e| match e {
        AuditError::NotFound => AgentError::NotFound,
        other => AgentError::Audit(other),
    })?;

    if audit.is_archived {
        return Err(AgentError::NotFound);
    }

    // Update last_connected_at
    let now = chrono::Utc::now().naive_utc();
    diesel::update(audit_agents::table.filter(audit_agents::id.eq(&agent.id)))
        .set(audit_agents::last_connected_at.eq(now))
        .execute(&mut conn)?;

    Ok(agent)
}
