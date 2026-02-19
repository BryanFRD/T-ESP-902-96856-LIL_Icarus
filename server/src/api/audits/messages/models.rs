use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

use crate::api::audits::tickets::models::TicketUser;
use crate::api::users::User;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::audit_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct AuditMessage {
    pub id: String,
    pub audit_id: String,
    pub author_id: String,
    pub content: String,
    pub content_nonce: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AuditMessage {
    #[must_use]
    pub fn new(
        audit_id: String,
        author_id: String,
        content: String,
        content_nonce: Vec<u8>,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            audit_id,
            author_id,
            content,
            content_nonce,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::audit_message_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct AuditMessageAttachment {
    pub id: String,
    pub message_id: String,
    pub uploaded_by: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: NaiveDateTime,
}

impl AuditMessageAttachment {
    #[must_use]
    pub fn new(
        message_id: String,
        uploaded_by: String,
        file_name: String,
        file_path: String,
        file_size: i64,
        mime_type: String,
    ) -> Self {
        Self {
            id: Ulid::new().to_string(),
            message_id,
            uploaded_by,
            file_name,
            file_path,
            file_size,
            mime_type,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuditMessagePayload {
    #[validate(length(min = 1, max = 4000))]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditMessageResponse {
    pub id: String,
    pub audit_id: String,
    pub author: TicketUser,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl AuditMessageResponse {
    #[must_use]
    pub fn from_message_and_author(message: AuditMessage, author: User, content: String) -> Self {
        Self {
            id: message.id,
            audit_id: message.audit_id,
            author: TicketUser::from(author),
            content,
            created_at: message.created_at.to_string(),
            updated_at: message.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditMessageAttachmentResponse {
    pub id: String,
    pub message_id: String,
    pub uploaded_by: TicketUser,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: String,
}

impl AuditMessageAttachmentResponse {
    #[must_use]
    pub fn from_attachment_and_user(attachment: AuditMessageAttachment, user: User) -> Self {
        Self {
            id: attachment.id,
            message_id: attachment.message_id,
            uploaded_by: TicketUser::from(user),
            file_name: attachment.file_name,
            file_size: attachment.file_size,
            mime_type: attachment.mime_type,
            created_at: attachment.created_at.to_string(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct AuditMessageAttachmentForm {
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    pub file: String,
}
