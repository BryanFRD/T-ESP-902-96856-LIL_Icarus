use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use std::path::PathBuf;
use tokio::fs;
use tracing::error;

use crate::{
    api::auth::JwtClaims,
    helpers::{internal_error, not_found_error},
    state::GlobalState,
};

use crate::api::audits::tickets::error::TicketError;

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/attachments",
    tag = "Ticket Attachments",
    responses(
        (status = 200, description = "Attachments retrieved successfully", body = Vec<super::models::AttachmentResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
pub async fn list_attachments(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::services::list_attachments(&state, &audit_id, &ticket_id) {
        Ok(attachments) => (StatusCode::OK, Json(attachments)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(e) => internal_error("Failed to list attachments", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/attachments",
    tag = "Ticket Attachments",
    request_body(content = super::models::AttachmentForm, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Attachment uploaded successfully", body = super::models::AttachmentResponse),
        (status = 400, description = "File too large or invalid"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Ticket not found"),
    ),
)]
#[axum::debug_handler]
pub async fn upload_attachment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, ticket_id)): Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    match super::services::upload_attachment(
        &state,
        &audit_id,
        &ticket_id,
        &jwt_user.sub.to_string(),
        multipart,
    )
    .await
    {
        Ok(attachment) => (StatusCode::CREATED, Json(attachment)).into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::FileTooLarge) => (
            StatusCode::BAD_REQUEST,
            Json("error.ticket.attachment.fileTooLarge"),
        )
            .into_response(),
        Err(TicketError::InvalidFileType) => (
            StatusCode::BAD_REQUEST,
            Json("error.ticket.attachment.invalidFileType"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to upload attachment", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/attachments/{attachment_id}",
    tag = "Ticket Attachments",
    responses(
        (status = 200, description = "File downloaded successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found"),
    ),
)]
pub async fn download_attachment(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id, attachment_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let attachment =
        match super::services::get_attachment(&state, &audit_id, &ticket_id, &attachment_id) {
            Ok(att) => att,
            Err(TicketError::NotFound) => return not_found_error("error.ticket.notFound"),
            Err(TicketError::AttachmentNotFound) => {
                return not_found_error("error.ticket.attachment.notFound");
            }
            Err(e) => return internal_error("Failed to get attachment", e),
        };

    let file_path = PathBuf::from(&attachment.file_path);
    if !file_path.exists() {
        return not_found_error("error.ticket.attachment.fileNotFound");
    }

    match fs::read(&file_path).await {
        Ok(contents) => {
            let headers = [
                (http::header::CONTENT_TYPE, attachment.mime_type),
                (
                    http::header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", attachment.file_name),
                ),
            ];
            (StatusCode::OK, headers, contents).into_response()
        }
        Err(e) => {
            error!("Failed to read attachment file: {}", e);
            not_found_error("error.ticket.attachment.fileNotFound")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/tickets/{ticket_id}/attachments/{attachment_id}",
    tag = "Ticket Attachments",
    responses(
        (status = 204, description = "Attachment deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Attachment not found"),
    ),
)]
pub async fn delete_attachment(
    State(state): State<GlobalState>,
    _jwt_user: JwtClaims,
    Path((audit_id, ticket_id, attachment_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match super::services::delete_attachment(&state, &audit_id, &ticket_id, &attachment_id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(TicketError::NotFound) => not_found_error("error.ticket.notFound"),
        Err(TicketError::AttachmentNotFound) => not_found_error("error.ticket.attachment.notFound"),
        Err(e) => internal_error("Failed to delete attachment", e),
    }
}
