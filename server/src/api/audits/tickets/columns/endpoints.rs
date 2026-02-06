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

use super::models::{CreateColumnPayload, ReorderColumnsPayload, UpdateColumnPayload};
use crate::api::audits::tickets::error::TicketError;

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/columns",
    tag = "Kanban Columns",
    responses(
        (status = 200, description = "Columns retrieved successfully", body = Vec<super::models::ColumnResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn list_columns(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
) -> impl IntoResponse {
    match super::services::list_columns(&state, &audit_id) {
        Ok(columns) => (StatusCode::OK, Json(columns)).into_response(),
        Err(e) => internal_error("Failed to list columns", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/columns",
    tag = "Kanban Columns",
    request_body = CreateColumnPayload,
    responses(
        (status = 201, description = "Column created successfully", body = super::models::ColumnResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Duplicate column name"),
    ),
)]
pub async fn create_column(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<CreateColumnPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::create_column(&state, &audit_id, &payload) {
        Ok(column) => (StatusCode::CREATED, Json(column)).into_response(),
        Err(TicketError::DuplicateColumnName) => (
            StatusCode::CONFLICT,
            Json("error.ticket.column.duplicateName"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to create column", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}/columns/{column_id}",
    tag = "Kanban Columns",
    request_body = UpdateColumnPayload,
    responses(
        (status = 200, description = "Column updated successfully", body = super::models::ColumnResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Column not found"),
        (status = 409, description = "Duplicate column name"),
    ),
)]
pub async fn update_column(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, column_id)): Path<(String, String)>,
    Json(payload): Json<UpdateColumnPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::update_column(&state, &audit_id, &column_id, &payload) {
        Ok(column) => (StatusCode::OK, Json(column)).into_response(),
        Err(TicketError::ColumnNotFound) => not_found_error("error.ticket.column.notFound"),
        Err(TicketError::DuplicateColumnName) => (
            StatusCode::CONFLICT,
            Json("error.ticket.column.duplicateName"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to update column", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/columns/{column_id}",
    tag = "Kanban Columns",
    responses(
        (status = 204, description = "Column deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Column not found"),
        (status = 409, description = "Column has tickets"),
    ),
)]
pub async fn delete_column(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, column_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::delete_column(&state, &audit_id, &column_id) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::ColumnNotFound) => not_found_error("error.ticket.column.notFound"),
        Err(TicketError::ColumnHasTickets) => {
            (StatusCode::CONFLICT, Json("error.ticket.column.hasTickets")).into_response()
        }
        Err(e) => internal_error("Failed to delete column", e),
    }
}

#[utoipa::path(
    put,
    path = "/v1/audits/{audit_id}/columns/reorder",
    tag = "Kanban Columns",
    request_body = ReorderColumnsPayload,
    responses(
        (status = 200, description = "Columns reordered successfully", body = Vec<super::models::ColumnResponse>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn reorder_columns(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<ReorderColumnsPayload>,
) -> impl IntoResponse {
    match super::services::reorder_columns(&state, &audit_id, &payload) {
        Ok(columns) => (StatusCode::OK, Json(columns)).into_response(),
        Err(e) => internal_error("Failed to reorder columns", e),
    }
}
