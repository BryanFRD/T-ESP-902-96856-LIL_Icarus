use serde::Serialize;
use utoipa::ToSchema;

use super::priority::TicketPriority;
use crate::api::audits::tickets::labels::models::LabelResponse;
use crate::api::users::User;

use super::ticket::Ticket;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketUser {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
}

impl From<User> for TicketUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            avatar_url: user.avatar_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketResponse {
    pub id: String,
    pub audit_id: String,
    pub column_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
    pub assignee: Option<TicketUser>,
    pub position: i32,
    pub due_date: Option<String>,
    pub created_by: TicketUser,
    pub labels: Vec<LabelResponse>,
    pub subtask_count: i64,
    pub comment_count: i64,
    pub attachment_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl TicketResponse {
    #[must_use]
    pub fn from_ticket(
        ticket: Ticket,
        assignee: Option<User>,
        created_by: User,
        labels: Vec<LabelResponse>,
        subtask_count: i64,
        comment_count: i64,
        attachment_count: i64,
    ) -> Self {
        Self {
            id: ticket.id,
            audit_id: ticket.audit_id,
            column_id: ticket.column_id,
            parent_id: ticket.parent_id,
            title: ticket.title,
            description: ticket.description,
            priority: ticket.priority,
            assignee: assignee.map(TicketUser::from),
            position: ticket.position,
            due_date: ticket.due_date.map(|d| d.to_string()),
            created_by: TicketUser::from(created_by),
            labels,
            subtask_count,
            comment_count,
            attachment_count,
            created_at: ticket.created_at.to_string(),
            updated_at: ticket.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketSummary {
    pub id: String,
    pub column_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub priority: TicketPriority,
    pub assignee: Option<TicketUser>,
    pub position: i32,
    pub due_date: Option<String>,
    pub labels: Vec<LabelResponse>,
    pub subtask_count: i64,
    pub comment_count: i64,
    pub attachment_count: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BoardColumn {
    pub id: String,
    pub name: String,
    pub position: i32,
    pub tickets: Vec<TicketSummary>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BoardResponse {
    pub audit_id: String,
    pub columns: Vec<BoardColumn>,
    pub labels: Vec<LabelResponse>,
}
