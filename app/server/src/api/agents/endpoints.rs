use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use ulid::Ulid;
use validator::Validate;

use crate::{
    api::auth::JwtClaims,
    helpers::{bad_request_error, forbidden_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::{
    AgentError, AgentResponse, AgentTokenResponse, CreateAgentPayload, KeyloggerRequestPayload,
    KeyloggerResponse, ShellRequestPayload, ShellStreamResponse,
};

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/agents",
    tag = "Agents",
    request_body = CreateAgentPayload,
    responses(
        (status = 201, description = "Agent created successfully", body = AgentTokenResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Audit not found"),
        (status = 409, description = "Agent name already exists"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn create_agent(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
    Json(payload): Json<CreateAgentPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::create_agent(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        payload,
        jwt_user.is_admin(),
    ) {
        Ok(response) => (StatusCode::CREATED, Json(response)).into_response(),
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(AgentError::NameAlreadyExists) => {
            (StatusCode::CONFLICT, Json("error.agent.nameAlreadyExists")).into_response()
        }
        Err(AgentError::Audit(crate::api::audits::AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(e) => internal_error("Failed to create agent", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/agents/download",
    tag = "Agents",
    responses(
        (status = 200, description = "Agent binary downloaded", content_type = "application/octet-stream"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Audit or binary not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn download_agent(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    headers: http::HeaderMap,
    Path(audit_id): Path<Ulid>,
) -> impl IntoResponse {
    let host_header = headers
        .get(http::header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost");
    let server_host = host_header.split(':').next().unwrap_or(host_header);

    match super::services::download_agent(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
        server_host,
    ) {
        Ok(binary) => {
            let headers = [
                (
                    http::header::CONTENT_TYPE,
                    "application/octet-stream".to_string(),
                ),
                (
                    http::header::CONTENT_DISPOSITION,
                    "attachment; filename=\"agent.exe\"".to_string(),
                ),
            ];
            (StatusCode::OK, headers, binary).into_response()
        }
        Err(AgentError::BinaryNotFound) => not_found_error("error.agent.binaryNotFound"),
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(AgentError::Audit(crate::api::audits::AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(e) => internal_error("Failed to download agent", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/agents",
    tag = "Agents",
    responses(
        (status = 200, description = "Agents listed successfully", body = Vec<AgentResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Audit not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn list_agents(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
) -> impl IntoResponse {
    match super::services::list_agents(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(agents) => (StatusCode::OK, Json(agents)).into_response(),
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(AgentError::Audit(crate::api::audits::AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(e) => internal_error("Failed to list agents", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/agents/{agent_id}",
    tag = "Agents",
    responses(
        (status = 204, description = "Agent revoked successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Agent not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn revoke_agent(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, agent_id)): Path<(Ulid, Ulid)>,
) -> impl IntoResponse {
    match super::services::revoke_agent(
        &state,
        &audit_id.to_string(),
        &agent_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AgentError::NotFound) => not_found_error("error.agent.notFound"),
        Err(AgentError::Revoked) => {
            (StatusCode::CONFLICT, Json("error.agent.alreadyRevoked")).into_response()
        }
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(e) => internal_error("Failed to revoke agent", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/agents/{agent_id}/shell",
    tag = "Agents",
    request_body = ShellRequestPayload,
    responses(
        (status = 200, description = "Shell session requested", body = ShellStreamResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Agent not found"),
        (status = 409, description = "Agent not online"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
#[axum::debug_handler]
pub async fn request_shell(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, agent_id)): Path<(Ulid, Ulid)>,
    Json(payload): Json<ShellRequestPayload>,
) -> impl IntoResponse {
    match super::services::request_shell(
        &state,
        &audit_id.to_string(),
        &agent_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
        payload,
    )
    .await
    {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(AgentError::NotFound) => not_found_error("error.agent.notFound"),
        Err(AgentError::AgentNotOnline) => {
            (StatusCode::CONFLICT, Json("error.agent.notOnline")).into_response()
        }
        Err(AgentError::Revoked) => {
            (StatusCode::CONFLICT, Json("error.agent.alreadyRevoked")).into_response()
        }
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(e) => internal_error("Failed to request shell", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/agents/{agent_id}/keylogger",
    tag = "Agents",
    request_body = KeyloggerRequestPayload,
    responses(
        (status = 200, description = "Keylogger request sent", body = KeyloggerResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Agent not found"),
        (status = 409, description = "Agent not online"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
#[axum::debug_handler]
pub async fn request_keylogger(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, agent_id)): Path<(Ulid, Ulid)>,
    Json(payload): Json<KeyloggerRequestPayload>,
) -> impl IntoResponse {
    match super::services::request_keylogger(
        &state,
        &audit_id.to_string(),
        &agent_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
        payload,
    )
    .await
    {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(AgentError::NotFound) => not_found_error("error.agent.notFound"),
        Err(AgentError::AgentNotOnline) => {
            (StatusCode::CONFLICT, Json("error.agent.notOnline")).into_response()
        }
        Err(AgentError::Revoked) => {
            (StatusCode::CONFLICT, Json("error.agent.alreadyRevoked")).into_response()
        }
        Err(AgentError::MustBeOwnerOrAdmin) => forbidden_error("error.agent.mustBeOwnerOrAdmin"),
        Err(e) => internal_error("Failed to request keylogger", e),
    }
}
