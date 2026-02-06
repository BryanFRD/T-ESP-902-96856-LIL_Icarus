use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::StatusCode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    api::{auth::JwtClaims, users::UserRole},
    helpers::{bad_request_error, forbidden_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::error::TicketError;
use super::models::{CreateTicketPayload, MoveTicketPayload, UpdateTicketPayload};

#[derive(Debug, Deserialize)]
pub struct TicketFilters {
    pub column: Option<String>,
    pub priority: Option<String>,
    pub assignee: Option<String>,
}

fn handle_auth_error(e: TicketError) -> axum::response::Response {
    match e {
        TicketError::AuditArchived => forbidden_error("error.audit.archived"),
        TicketError::MustBeTeamMember => forbidden_error("error.ticket.mustBeTeamMember"),
        TicketError::MustBeOwnerOrAdmin => forbidden_error("error.ticket.mustBeOwnerOrAdmin"),
        TicketError::ColumnAuditMismatch => bad_request_error("error.ticket.column.auditMismatch"),
        other => internal_error("Operation failed", other),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/board",
    tag = "Tickets",
    responses(
        (status = 200, description = "Board retrieved successfully", body = super::models::BoardResponse),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn get_board(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
) -> impl IntoResponse {
    match super::services::get_board(&state, &audit_id) {
        Ok(board) => (StatusCode::OK, Json(board)).into_response(),
        Err(e) => internal_error("Failed to get board", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets",
    tag = "Tickets",
    params(
        ("column" = Option<String>, Query, description = "Filter by column ID"),
        ("priority" = Option<String>, Query, description = "Filter by priority"),
        ("assignee" = Option<String>, Query, description = "Filter by assignee ID"),
    ),
    responses(
        (status = 200, description = "Tickets retrieved successfully", body = Vec<super::models::Ticket>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn list_tickets(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Query(filters): Query<TicketFilters>,
) -> impl IntoResponse {
    match super::services::list_tickets(
        &state,
        &audit_id,
        filters.column.as_deref(),
        filters.priority.as_deref(),
        filters.assignee.as_deref(),
    ) {
        Ok(tickets) => (StatusCode::OK, Json(tickets)).into_response(),
        Err(e) => internal_error("Failed to list tickets", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/tickets",
    tag = "Tickets",
    request_body = CreateTicketPayload,
    responses(
        (status = 201, description = "Ticket created successfully", body = super::models::Ticket),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Column or parent not found"),
    ),
)]
pub async fn create_ticket(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<CreateTicketPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::create_ticket(&state, &audit_id, &jwt_user.sub.to_string(), &payload) {
        Ok(ticket) => (StatusCode::CREATED, Json(ticket)).into_response(),
        Err(TicketError::ColumnNotFound) => not_found_error("error.ticket.column.notFound"),
        Err(TicketError::ParentNotFound) => not_found_error("error.ticket.parent.notFound"),
        Err(e @ (TicketError::AuditArchived | TicketError::MustBeTeamMember)) => {
            handle_auth_error(e)
        }
        Err(e) => internal_error("Failed to create ticket", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}",
    tag = "Tickets",
    responses(
        (status = 200, description = "Ticket retrieved successfully", body = super::models::TicketResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn get_ticket(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::get_ticket(&state, &audit_id, &ticket_id) {
        Ok(ticket) => (StatusCode::OK, Json(ticket)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(e) => internal_error("Failed to get ticket", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}",
    tag = "Tickets",
    request_body = UpdateTicketPayload,
    responses(
        (status = 200, description = "Ticket updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn update_ticket(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
    Json(payload): Json<UpdateTicketPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::services::update_ticket(
        &state,
        &audit_id,
        &ticket_id,
        &jwt_user.sub.to_string(),
        &payload,
    ) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::SelfParent) => bad_request_error("error.ticket.selfParent"),
        Err(TicketError::CircularParent) => bad_request_error("error.ticket.circularParent"),
        Err(TicketError::ParentNotFound) => not_found_error("error.ticket.parent.notFound"),
        Err(e @ (TicketError::AuditArchived | TicketError::MustBeTeamMember)) => {
            handle_auth_error(e)
        }
        Err(e) => internal_error("Failed to update ticket", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}",
    tag = "Tickets",
    responses(
        (status = 204, description = "Ticket deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn delete_ticket(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let is_admin = jwt_user.role == UserRole::Admin;
    match super::services::delete_ticket(
        &state,
        &audit_id,
        &ticket_id,
        &jwt_user.sub.to_string(),
        is_admin,
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(
            e @ (TicketError::AuditArchived
            | TicketError::MustBeTeamMember
            | TicketError::MustBeOwnerOrAdmin),
        ) => handle_auth_error(e),
        Err(e) => internal_error("Failed to delete ticket", e),
    }
}

#[utoipa::path(
    put,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/move",
    tag = "Tickets",
    request_body = MoveTicketPayload,
    responses(
        (status = 200, description = "Ticket moved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Ticket or column not found"),
    ),
)]
pub async fn move_ticket(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
    Json(payload): Json<MoveTicketPayload>,
) -> impl IntoResponse {
    match super::services::move_ticket(
        &state,
        &audit_id,
        &ticket_id,
        &jwt_user.sub.to_string(),
        &payload,
    ) {
        Ok(()) => StatusCode::OK.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::ColumnNotFound) => not_found_error("error.ticket.column.notFound"),
        Err(
            e @ (TicketError::AuditArchived
            | TicketError::MustBeTeamMember
            | TicketError::ColumnAuditMismatch),
        ) => handle_auth_error(e),
        Err(e) => internal_error("Failed to move ticket", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/subtasks",
    tag = "Tickets",
    responses(
        (status = 200, description = "Subtasks retrieved successfully", body = Vec<super::models::Ticket>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn list_subtasks(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::list_subtasks(&state, &audit_id, &ticket_id) {
        Ok(subtasks) => (StatusCode::OK, Json(subtasks)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(e) => internal_error("Failed to list subtasks", e),
    }
}
