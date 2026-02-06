use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use validator::Validate;

use crate::{
    api::auth::JwtClaims,
    helpers::{bad_request_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::models::{CreateCommentPayload, UpdateCommentPayload};
use crate::api::audits::tickets::error::TicketError;

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/comments",
    tag = "Ticket Comments",
    responses(
        (status = 200, description = "Comments retrieved successfully", body = Vec<super::models::CommentResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn list_comments(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::list_comments(&state, &audit_id, &ticket_id) {
        Ok(comments) => (StatusCode::OK, Json(comments)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(e) => internal_error("Failed to list comments", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/comments",
    tag = "Ticket Comments",
    request_body = CreateCommentPayload,
    responses(
        (status = 201, description = "Comment created successfully", body = super::models::CommentResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn create_comment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
    Json(payload): Json<CreateCommentPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::create_comment(
        &state,
        &audit_id,
        &ticket_id,
        &jwt_user.sub.to_string(),
        &payload,
    ) {
        Ok(comment) => (StatusCode::CREATED, Json(comment)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(e) => internal_error("Failed to create comment", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/comments/{comment_id}",
    tag = "Ticket Comments",
    request_body = UpdateCommentPayload,
    responses(
        (status = 200, description = "Comment updated successfully", body = super::models::CommentResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not comment author"),
        (status = 404, description = "Comment not found"),
    ),
)]
pub async fn update_comment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id, comment_id)): Path<(String, String, String)>,
    Json(payload): Json<UpdateCommentPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::update_comment(
        &state,
        &audit_id,
        &ticket_id,
        &comment_id,
        &jwt_user.sub.to_string(),
        &payload,
    ) {
        Ok(comment) => (StatusCode::OK, Json(comment)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::CommentNotFound) => not_found_error("error.ticket.comment.notFound"),
        Err(TicketError::NotCommentAuthor) => (
            StatusCode::FORBIDDEN,
            Json("error.ticket.comment.notAuthor"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to update comment", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/comments/{comment_id}",
    tag = "Ticket Comments",
    responses(
        (status = 204, description = "Comment deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Comment not found"),
    ),
)]
pub async fn delete_comment(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id, comment_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match super::services::delete_comment(&state, &audit_id, &ticket_id, &comment_id) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::CommentNotFound) => not_found_error("error.ticket.comment.notFound"),
        Err(e) => internal_error("Failed to delete comment", e),
    }
}
