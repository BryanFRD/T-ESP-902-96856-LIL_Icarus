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
#[diesel(table_name = crate::schema::ticket_comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct TicketComment {
    pub id: String,
    pub ticket_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TicketComment {
    #[must_use]
    pub fn new(ticket_id: String, author_id: String, content: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            ticket_id,
            author_id,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentPayload {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentPayload {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub id: String,
    pub ticket_id: String,
    pub author: TicketUser,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl CommentResponse {
    #[must_use]
    pub fn from_comment_and_author(comment: TicketComment, author: User) -> Self {
        Self {
            id: comment.id,
            ticket_id: comment.ticket_id,
            author: TicketUser::from(author),
            content: comment.content,
            created_at: comment.created_at.to_string(),
            updated_at: comment.updated_at.to_string(),
        }
    }
}
