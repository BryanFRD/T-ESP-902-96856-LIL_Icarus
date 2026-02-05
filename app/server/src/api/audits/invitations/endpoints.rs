use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use tower_cookies::Cookies;
use tracing::error;
use validator::Validate;

use crate::{api::auth::JwtClaims, helpers::create_auth_cookie, state::GlobalState};

use super::{
    AcceptAuditInvitationPayload, AcceptInvitationResponse, AuditInvitationError,
    AuditInvitationReturn, CheckInvitationResponse, CreateAuditInvitationPayload,
};

fn extract_optional_jwt(cookies: &Cookies) -> Option<JwtClaims> {
    let auth_cookie = cookies.get("auth")?;
    let jwt_secret = std::env::var("JWT_SECRET").ok()?;
    let jwt = JwtClaims::from_jwt(auth_cookie.value(), &jwt_secret);

    if jwt.is_valid() { Some(jwt) } else { None }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/invite",
    tag = "Audits",
    request_body = CreateAuditInvitationPayload,
    responses(
        (status = 201, description = "Invitation created successfully", body = AuditInvitationReturn),
        (status = 400, description = "Invalid request payload"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Not owner/moderator of team"),
        (status = 404, description = "Audit not found"),
        (status = 409, description = "Email already in audit"),
    ),
)]
pub async fn create_invitation(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<CreateAuditInvitationPayload>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json("error.validation.invalidEmail"),
        )
            .into_response();
    }

    match super::create_audit_invitation(
        &state,
        &audit_id,
        &jwt_user.sub.to_string(),
        payload.client_email,
        payload.expires_at,
        jwt_user.is_admin(),
    )
    .await
    {
        Ok(invitation) => (
            StatusCode::CREATED,
            Json(AuditInvitationReturn::from(invitation)),
        )
            .into_response(),
        Err(AuditInvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(AuditInvitationError::AuditError(e)) if e.to_string().contains("not found") => {
            (StatusCode::NOT_FOUND, Json("error.audit.notFound")).into_response()
        }
        Err(AuditInvitationError::AuditError(e)) if e.to_string().contains("archived") => {
            (StatusCode::BAD_REQUEST, Json("error.audit.archived")).into_response()
        }
        Err(AuditInvitationError::EmailAlreadyInAudit) => (
            StatusCode::CONFLICT,
            Json("error.invitation.emailAlreadyInAudit"),
        )
            .into_response(),
        Err(e) => {
            error!("Failed to create audit invitation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/invite",
    tag = "Audits",
    responses(
        (status = 200, description = "List of invitations", body = Vec<AuditInvitationReturn>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn list_invitations(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
) -> impl IntoResponse {
    match super::list_audit_invitations(
        &state,
        &audit_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(invitations) => {
            let items: Vec<AuditInvitationReturn> = invitations
                .into_iter()
                .map(AuditInvitationReturn::from)
                .collect();
            Json(items).into_response()
        }
        Err(AuditInvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(AuditInvitationError::AuditError(e)) if e.to_string().contains("not found") => {
            (StatusCode::NOT_FOUND, Json("error.audit.notFound")).into_response()
        }
        Err(e) => {
            error!("Failed to list invitations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/invite/{invitation_id}",
    tag = "Audits",
    responses(
        (status = 204, description = "Invitation canceled successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Invitation not found"),
    ),
)]
pub async fn cancel_invitation(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, invitation_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::cancel_audit_invitation(
        &state,
        &audit_id,
        &invitation_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditInvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(AuditInvitationError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.invitation.notFound")).into_response()
        }
        Err(e) => {
            error!("Failed to cancel invitation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/invite/{token}/check",
    tag = "Audits",
    responses(
        (status = 200, description = "Invitation details", body = CheckInvitationResponse),
        (status = 404, description = "Invitation not found"),
        (status = 410, description = "Invitation expired, canceled, or already used"),
    ),
)]
pub async fn check_invitation(
    State(state): State<GlobalState>,
    Path(token): Path<String>,
) -> impl IntoResponse {
    match super::check_invitation(&state, &token) {
        Ok(response) => Json(response).into_response(),
        Err(AuditInvitationError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.invitation.notFound")).into_response()
        }
        Err(AuditInvitationError::Expired) => {
            (StatusCode::GONE, Json("error.invitation.expired")).into_response()
        }
        Err(AuditInvitationError::Canceled) => {
            (StatusCode::GONE, Json("error.invitation.canceled")).into_response()
        }
        Err(AuditInvitationError::AlreadyUsed) => {
            (StatusCode::GONE, Json("error.invitation.alreadyUsed")).into_response()
        }
        Err(e) => {
            error!("Failed to check invitation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/invite/{token}/accept",
    tag = "Audits",
    request_body = AcceptAuditInvitationPayload,
    responses(
        (status = 200, description = "Invitation accepted successfully", body = AcceptInvitationResponse),
        (status = 400, description = "Missing account details or validation error"),
        (status = 401, description = "Authentication required for existing user"),
        (status = 403, description = "Email mismatch"),
        (status = 404, description = "Invitation not found"),
        (status = 409, description = "Username already taken or email already in audit"),
        (status = 410, description = "Invitation expired, canceled, or already used"),
    ),
)]
pub async fn accept_invitation(
    State(state): State<GlobalState>,
    cookies: Cookies,
    Path(token): Path<String>,
    Json(payload): Json<AcceptAuditInvitationPayload>,
) -> impl IntoResponse {
    let jwt_user = extract_optional_jwt(&cookies);

    if payload.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json("error.validation.passwordTooShort"),
        )
            .into_response();
    }

    match super::accept_audit_invitation(&state, &token, payload, jwt_user.as_ref()) {
        Ok(AcceptInvitationResponse::AccountCreated { jwt, audit_id, .. }) => {
            create_auth_cookie(jwt, &cookies);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "type": "accountCreated",
                    "auditId": audit_id,
                })),
            )
                .into_response()
        }
        Ok(AcceptInvitationResponse::AccountExists { audit_id }) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "type": "accountExists",
                "auditId": audit_id,
            })),
        )
            .into_response(),
        Err(AuditInvitationError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.invitation.notFound")).into_response()
        }
        Err(AuditInvitationError::Expired) => {
            (StatusCode::GONE, Json("error.invitation.expired")).into_response()
        }
        Err(AuditInvitationError::Canceled) => {
            (StatusCode::GONE, Json("error.invitation.canceled")).into_response()
        }
        Err(AuditInvitationError::AlreadyUsed) => {
            (StatusCode::GONE, Json("error.invitation.alreadyUsed")).into_response()
        }
        Err(AuditInvitationError::EmailMismatch) => (
            StatusCode::FORBIDDEN,
            Json("error.invitation.emailMismatch"),
        )
            .into_response(),
        Err(AuditInvitationError::AuthenticationRequired) => {
            (StatusCode::UNAUTHORIZED, Json("error.user.auth.required")).into_response()
        }
        Err(AuditInvitationError::MissingAccountDetails) => (
            StatusCode::BAD_REQUEST,
            Json("error.invitation.missingAccountDetails"),
        )
            .into_response(),
        Err(AuditInvitationError::UsernameAlreadyTaken) => {
            (StatusCode::CONFLICT, Json("error.user.usernameTaken")).into_response()
        }
        Err(AuditInvitationError::PasswordTooShort) => (
            StatusCode::BAD_REQUEST,
            Json("error.validation.passwordTooShort"),
        )
            .into_response(),
        Err(AuditInvitationError::EmailAlreadyInAudit) => (
            StatusCode::CONFLICT,
            Json("error.invitation.emailAlreadyInAudit"),
        )
            .into_response(),
        Err(e) => {
            error!("Failed to accept invitation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}
