use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;

use crate::api::audits::tickets::models::TicketUser;
use crate::api::users::User;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::ticket_attachments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct TicketAttachment {
    pub id: String,
    pub ticket_id: String,
    pub uploaded_by: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: NaiveDateTime,
}

impl TicketAttachment {
    #[must_use]
    pub fn new(
        ticket_id: String,
        uploaded_by: String,
        file_name: String,
        file_path: String,
        file_size: i64,
        mime_type: String,
    ) -> Self {
        Self {
            id: Ulid::new().to_string(),
            ticket_id,
            uploaded_by,
            file_name,
            file_path,
            file_size,
            mime_type,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentResponse {
    pub id: String,
    pub ticket_id: String,
    pub uploaded_by: TicketUser,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub created_at: String,
}

impl AttachmentResponse {
    #[must_use]
    pub fn from_attachment_and_user(attachment: TicketAttachment, user: User) -> Self {
        Self {
            id: attachment.id,
            ticket_id: attachment.ticket_id,
            uploaded_by: TicketUser::from(user),
            file_name: attachment.file_name,
            file_size: attachment.file_size,
            mime_type: attachment.mime_type,
            created_at: attachment.created_at.to_string(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct AttachmentForm {
    #[allow(dead_code)]
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    pub file: String,
}
