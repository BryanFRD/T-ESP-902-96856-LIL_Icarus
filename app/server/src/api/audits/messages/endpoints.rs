use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::StatusCode;
use std::path::PathBuf;
use tokio::fs;
use tracing::error;
use validator::Validate;

use crate::{
    api::auth::JwtClaims,
    helpers::{bad_request_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::error::AuditMessageError;
use super::models::{AuditMessageAttachmentForm, CreateAuditMessagePayload};
use super::services;
use crate::api::audits::AuditError;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/messages",
    tag = "Audit Messages",
    params(
        ("limit" = Option<i64>, Query, description = "Limit results (default: 100, max: 500)"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination"),
    ),
    responses(
        (status = 200, description = "Messages retrieved successfully", body = Vec<super::models::AuditMessageResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn list_messages(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Query(query): Query<MessageListQuery>,
) -> impl IntoResponse {
    match services::list_messages(
        &state,
        &audit_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
        query.limit,
        query.offset,
    ) {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(AuditMessageError::AuditArchived) => bad_request_error("error.audit.archived"),
        Err(AuditMessageError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => internal_error("Failed to list audit messages", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/messages",
    tag = "Audit Messages",
    request_body = CreateAuditMessagePayload,
    responses(
        (status = 201, description = "Message created successfully", body = super::models::AuditMessageResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Audit not found"),
    ),
)]
pub async fn create_message(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<String>,
    Json(payload): Json<CreateAuditMessagePayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match services::create_message(
        &state,
        &audit_id,
        &jwt_user.sub.to_string(),
        &payload,
        jwt_user.is_admin(),
    ) {
        Ok(message) => {
            if let Some(io) = state.io_opt() {
                let room_id = format!("audit:{audit_id}");
                if let Err(e) = io
                    .of("/v1/ws")
                    .expect("namespace /v1/ws not found")
                    .to(room_id)
                    .emit("audit:message", &message)
                    .await
                {
                    error!(audit_id = %audit_id, error = %e, "Failed to emit audit:message");
                }
            }
            (StatusCode::CREATED, Json(message)).into_response()
        }
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(AuditMessageError::AuditArchived) => bad_request_error("error.audit.archived"),
        Err(AuditMessageError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => internal_error("Failed to create audit message", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/messages/{message_id}/attachments",
    tag = "Audit Message Attachments",
    responses(
        (status = 200, description = "Attachments retrieved successfully", body = Vec<super::models::AuditMessageAttachmentResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Message not found"),
    ),
)]
pub async fn list_attachments(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, message_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match services::list_attachments(
        &state,
        &audit_id,
        &message_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(attachments) => (StatusCode::OK, Json(attachments)).into_response(),
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(AuditMessageError::AuditArchived) => bad_request_error("error.audit.archived"),
        Err(AuditMessageError::MessageNotFound) => not_found_error("error.audit.message.notFound"),
        Err(AuditMessageError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => internal_error("Failed to list message attachments", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/{audit_id}/messages/{message_id}/attachments",
    tag = "Audit Message Attachments",
    request_body(content = AuditMessageAttachmentForm, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Attachment uploaded successfully", body = super::models::AuditMessageAttachmentResponse),
        (status = 400, description = "File too large"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Message not found"),
    ),
)]
#[axum::debug_handler]
pub async fn upload_attachment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, message_id)): Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    match services::upload_attachment(
        &state,
        &audit_id,
        &message_id,
        &jwt_user.sub.to_string(),
        multipart,
        jwt_user.is_admin(),
    )
    .await
    {
        Ok(attachment) => {
            if let Some(io) = state.io_opt() {
                let room_id = format!("audit:{audit_id}");
                if let Err(e) = io
                    .of("/v1/ws")
                    .expect("namespace /v1/ws not found")
                    .to(room_id)
                    .emit("audit:message_attachment", &attachment)
                    .await
                {
                    error!(audit_id = %audit_id, error = %e, "Failed to emit audit:message_attachment");
                }
            }
            (StatusCode::CREATED, Json(attachment)).into_response()
        }
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(AuditMessageError::AuditArchived) => bad_request_error("error.audit.archived"),
        Err(AuditMessageError::MessageNotFound) => not_found_error("error.audit.message.notFound"),
        Err(AuditMessageError::FileTooLarge) => (
            StatusCode::BAD_REQUEST,
            Json("error.audit.message.attachment.fileTooLarge"),
        )
            .into_response(),
        Err(AuditMessageError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => internal_error("Failed to upload message attachment", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/messages/{message_id}/attachments/{attachment_id}",
    tag = "Audit Message Attachments",
    responses(
        (status = 200, description = "File downloaded successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Attachment not found"),
    ),
)]
pub async fn download_attachment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, message_id, attachment_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let attachment = match services::get_attachment(
        &state,
        &audit_id,
        &message_id,
        &attachment_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(att) => att,
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            return not_found_error("error.audit.notFound");
        }
        Err(AuditMessageError::AuditArchived) => return bad_request_error("error.audit.archived"),
        Err(AuditMessageError::MessageNotFound) => {
            return not_found_error("error.audit.message.notFound");
        }
        Err(AuditMessageError::AttachmentNotFound) => {
            return not_found_error("error.audit.message.attachment.notFound");
        }
        Err(AuditMessageError::NotAuthorized) => {
            return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
        }
        Err(e) => return internal_error("Failed to get message attachment", e),
    };

    let file_path = PathBuf::from(&attachment.file_path);
    if !file_path.exists() {
        return not_found_error("error.audit.message.attachment.fileNotFound");
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
            error!("Failed to read message attachment file: {}", e);
            not_found_error("error.audit.message.attachment.fileNotFound")
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/messages/{message_id}/attachments/{attachment_id}",
    tag = "Audit Message Attachments",
    responses(
        (status = 204, description = "Attachment deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Attachment not found"),
    ),
)]
pub async fn delete_attachment(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, message_id, attachment_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    match services::delete_attachment(
        &state,
        &audit_id,
        &message_id,
        &attachment_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    )
    .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditMessageError::Audit(AuditError::NotFound)) => {
            not_found_error("error.audit.notFound")
        }
        Err(AuditMessageError::AuditArchived) => bad_request_error("error.audit.archived"),
        Err(AuditMessageError::MessageNotFound) => not_found_error("error.audit.message.notFound"),
        Err(AuditMessageError::AttachmentNotFound) => {
            not_found_error("error.audit.message.attachment.notFound")
        }
        Err(AuditMessageError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => internal_error("Failed to delete message attachment", e),
    }
}
