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
    helpers::{bad_request_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::{AuditError, AuditResponse, CreateAuditPayload, UpdateAuditPayload};

#[utoipa::path(
    get,
    path = "/v1/audits/me",
    tag = "Audits",
    responses(
        (status = 200, description = "Audits retrieved successfully", body = Vec<AuditResponse>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn list_my_audits(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
) -> impl IntoResponse {
    match super::list_user_audits(&state, &jwt_user.sub.to_string()) {
        Ok(audits) => (StatusCode::OK, Json(audits)).into_response(),
        Err(e) => internal_error("Failed to list audits", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/me",
    tag = "Audits",
    request_body = CreateAuditPayload,
    responses(
        (status = 201, description = "Audit created successfully", body = AuditResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Team or company not found"),
    ),
)]
pub async fn create_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Json(payload): Json<CreateAuditPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::create_audit(
        &state,
        &jwt_user.sub.to_string(),
        payload,
        jwt_user.is_admin(),
    ) {
        Ok(audit) => (StatusCode::CREATED, Json(audit)).into_response(),
        Err(AuditError::TeamNotFound) => not_found_error("error.audit.teamNotFound"),
        Err(AuditError::CompanyNotFound) => not_found_error("error.audit.companyNotFound"),
        Err(AuditError::CompanyTeamMismatch) => {
            bad_request_error("error.audit.companyTeamMismatch")
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(AuditError::EndDateInPast) => bad_request_error("error.audit.endDateInPast"),
        Err(e) => internal_error("Failed to create audit", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}",
    tag = "Audits",
    request_body = UpdateAuditPayload,
    responses(
        (status = 200, description = "Audit updated successfully", body = AuditResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit archived or insufficient permissions"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn update_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
    Json(payload): Json<UpdateAuditPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    if payload.name.is_none()
        && payload.description.is_none()
        && payload.end_date.is_none()
        && !payload.clear_end_date
    {
        return bad_request_error("error.audit.update.noFieldProvided");
    }

    match super::update_audit(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        &payload,
        jwt_user.is_admin(),
    ) {
        Ok(audit) => (StatusCode::OK, Json(audit)).into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.archived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(AuditError::EndDateInPast) => bad_request_error("error.audit.endDateInPast"),
        Err(e) => internal_error("Failed to update audit", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}",
    tag = "Audits",
    responses(
        (status = 204, description = "Audit archived successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit already archived or insufficient permissions"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn archive_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
) -> impl IntoResponse {
    match super::archive_audit(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.alreadyArchived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to archive audit", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/clients/{user_id}",
    tag = "Audits",
    responses(
        (status = 204, description = "Client removed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit archived or insufficient permissions"),
        (status = 404, description = "Audit or client not found"),
    ),
)]
pub async fn remove_client(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, user_id)): Path<(Ulid, Ulid)>,
) -> impl IntoResponse {
    match super::remove_client_from_audit(
        &state,
        &audit_id.to_string(),
        &user_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::ClientNotInAudit) => not_found_error("error.audit.clientNotInAudit"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.archived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to remove client", e),
    }
}
