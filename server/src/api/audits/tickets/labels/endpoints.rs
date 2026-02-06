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

use super::models::{CreateLabelPayload, UpdateLabelPayload};
use crate::api::audits::tickets::error::TicketError;

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/labels",
    tag = "Ticket Labels",
    responses(
        (status = 200, description = "Labels retrieved successfully", body = Vec<super::models::LabelResponse>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn list_labels(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
) -> impl IntoResponse {
    match super::services::list_labels(&state, &audit_id) {
        Ok(labels) => (StatusCode::OK, Json(labels)).into_response(),
        Err(e) => internal_error("Failed to list labels", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/labels",
    tag = "Ticket Labels",
    request_body = CreateLabelPayload,
    responses(
        (status = 201, description = "Label created successfully", body = super::models::LabelResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Duplicate label name"),
    ),
)]
pub async fn create_label(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<CreateLabelPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::create_label(&state, &audit_id, &payload) {
        Ok(label) => (StatusCode::CREATED, Json(label)).into_response(),
        Err(TicketError::DuplicateLabelName) => (
            StatusCode::CONFLICT,
            Json("error.ticket.label.duplicateName"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to create label", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}/labels/{label_id}",
    tag = "Ticket Labels",
    request_body = UpdateLabelPayload,
    responses(
        (status = 200, description = "Label updated successfully", body = super::models::LabelResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Label not found"),
        (status = 409, description = "Duplicate label name"),
    ),
)]
pub async fn update_label(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, label_id)): Path<(String, String)>,
    Json(payload): Json<UpdateLabelPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::update_label(&state, &audit_id, &label_id, &payload) {
        Ok(label) => (StatusCode::OK, Json(label)).into_response(),
        Err(TicketError::LabelNotFound) => not_found_error("error.ticket.label.notFound"),
        Err(TicketError::DuplicateLabelName) => (
            StatusCode::CONFLICT,
            Json("error.ticket.label.duplicateName"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to update label", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/labels/{label_id}",
    tag = "Ticket Labels",
    responses(
        (status = 204, description = "Label deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Label not found"),
    ),
)]
pub async fn delete_label(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, label_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::delete_label(&state, &audit_id, &label_id) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::LabelNotFound) => not_found_error("error.ticket.label.notFound"),
        Err(e) => internal_error("Failed to delete label", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/labels/{label_id}",
    tag = "Ticket Labels",
    responses(
        (status = 201, description = "Label attached successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket or label not found"),
        (status = 409, description = "Label already attached"),
    ),
)]
pub async fn attach_label(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id, label_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match super::services::attach_label(&state, &audit_id, &ticket_id, &label_id) {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::LabelNotFound) => not_found_error("error.ticket.label.notFound"),
        Err(TicketError::LabelAlreadyAttached) => (
            StatusCode::CONFLICT,
            Json("error.ticket.label.alreadyAttached"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to attach label", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/labels/{label_id}",
    tag = "Ticket Labels",
    responses(
        (status = 204, description = "Label detached successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket or label not found"),
    ),
)]
pub async fn detach_label(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id, label_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match super::services::detach_label(&state, &audit_id, &ticket_id, &label_id) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::LabelNotAttached) => not_found_error("error.ticket.label.notAttached"),
        Err(e) => internal_error("Failed to detach label", e),
    }
}
