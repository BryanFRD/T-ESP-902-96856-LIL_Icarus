use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;

use super::priority::TicketPriority;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::tickets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: String,
    pub audit_id: String,
    pub column_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
    pub assignee_id: Option<String>,
    pub position: i32,
    pub due_date: Option<NaiveDateTime>,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Ticket {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        audit_id: String,
        column_id: String,
        title: String,
        description: Option<String>,
        priority: TicketPriority,
        assignee_id: Option<String>,
        position: i32,
        due_date: Option<NaiveDateTime>,
        parent_id: Option<String>,
        created_by: String,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            audit_id,
            column_id,
            parent_id,
            title,
            description,
            priority,
            assignee_id,
            position,
            due_date,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}
