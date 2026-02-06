use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema,
)]
#[diesel(table_name = crate::schema::ticket_labels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct TicketLabel {
    pub id: String,
    pub audit_id: String,
    pub name: String,
    pub color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TicketLabel {
    #[must_use]
    pub fn new(audit_id: String, name: String, color: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            audit_id,
            name,
            color,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::tickets_labels)]
#[diesel(primary_key(ticket_id, label_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TicketLabelAssignment {
    pub ticket_id: String,
    pub label_id: String,
    pub created_at: NaiveDateTime,
}

impl TicketLabelAssignment {
    #[must_use]
    pub fn new(ticket_id: String, label_id: String) -> Self {
        Self {
            ticket_id,
            label_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabelPayload {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLabelPayload {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LabelResponse {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TicketLabel> for LabelResponse {
    fn from(label: TicketLabel) -> Self {
        Self {
            id: label.id,
            name: label.name,
            color: label.color,
            created_at: label.created_at.to_string(),
            updated_at: label.updated_at.to_string(),
        }
    }
}
