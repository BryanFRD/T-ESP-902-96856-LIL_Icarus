use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::kanban_columns)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct KanbanColumn {
    pub id: String,
    pub audit_id: String,
    pub name: String,
    pub position: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl KanbanColumn {
    #[must_use]
    pub fn new(audit_id: String, name: String, position: i32) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            audit_id,
            name,
            position,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateColumnPayload {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateColumnPayload {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReorderColumnsPayload {
    /// Ordered list of column IDs in the desired order
    pub column_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColumnResponse {
    pub id: String,
    pub name: String,
    pub position: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl From<KanbanColumn> for ColumnResponse {
    fn from(col: KanbanColumn) -> Self {
        Self {
            id: col.id,
            name: col.name,
            position: col.position,
            created_at: col.created_at.to_string(),
            updated_at: col.updated_at.to_string(),
        }
    }
}
