use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use tracing::error;

use crate::api::auth::JwtClaims;
use crate::state::GlobalState;

use super::{CreateInvitationPayload, InvitationError, InvitationReturn};

#[utoipa::path(
    post,
    path = "/v1/teams/{team_id}/invitations",
    tag = "Teams",
    request_body = CreateInvitationPayload,
    responses(
        (status = 201, description = "Invitation created", body = InvitationReturn),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team not found"),
    ),
)]
pub async fn create_invitation(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<String>,
    Json(payload): Json<CreateInvitationPayload>,
) -> impl IntoResponse {
    match super::create_invitation(
        &state,
        &team_id,
        &jwt_user.sub.to_string(),
        payload.max_uses,
        payload.expires_at,
        jwt_user.is_admin(),
    ) {
        Ok(inv) => (StatusCode::CREATED, Json(InvitationReturn::from(inv))).into_response(),
        Err(InvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(InvitationError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(e) => {
            error!("Failed to create invitation: {:?}", e);
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
    path = "/v1/teams/{team_id}/invitations",
    tag = "Teams",
    responses(
        (status = 200, description = "List invitations", body = [InvitationReturn]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team not found"),
    ),
)]
pub async fn list_invitations(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    match super::list_invitations(
        &state,
        &team_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(items) => {
            let items: Vec<InvitationReturn> =
                items.into_iter().map(InvitationReturn::from).collect();
            Json(items).into_response()
        }
        Err(InvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(InvitationError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
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
    path = "/v1/teams/{team_id}/invitations/{invitation_id}",
    tag = "Teams",
    responses(
        (status = 204, description = "Invitation canceled"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Invitation not found"),
    ),
)]
pub async fn delete_invitation(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, invitation_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::cancel_invitation(
        &state,
        &team_id,
        &invitation_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => (StatusCode::NO_CONTENT).into_response(),
        Err(InvitationError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(InvitationError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.invitation.notFound")).into_response()
        }
        Err(e) => {
            error!("Failed to delete invitation: {:?}", e);
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
    path = "/v1/teams/invitations/join/{token}",
    tag = "Teams",
    responses(
        (status = 200, description = "Joined team successfully"),
        (status = 404, description = "Invitation not found"),
        (status = 410, description = "Invitation expired"),
        (status = 409, description = "Invitation exhausted"),
        (status = 400, description = "Invitation canceled"),
    ),
)]
pub async fn consume_invitation(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(token): Path<String>,
) -> impl IntoResponse {
    match super::consume_invitation_by_token(&state, &token, &jwt_user.sub.to_string()) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(InvitationError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.invitation.notFound")).into_response()
        }
        Err(InvitationError::Expired) => {
            (StatusCode::GONE, Json("error.invitation.expired")).into_response()
        }
        Err(InvitationError::Exhausted) => {
            (StatusCode::CONFLICT, Json("error.invitation.exhausted")).into_response()
        }
        Err(InvitationError::Canceled) => {
            (StatusCode::BAD_REQUEST, Json("error.invitation.canceled")).into_response()
        }
        Err(e) => {
            error!("Failed to consume invitation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}
