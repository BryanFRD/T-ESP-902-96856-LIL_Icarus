use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::priority::TicketPriority;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketPayload {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
    pub column_id: String,
    pub assignee_id: Option<String>,
    pub parent_id: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime"
    )]
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTicketPayload {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
    pub assignee_id: Option<String>,
    #[serde(default)]
    pub clear_assignee: bool,
    pub parent_id: Option<String>,
    #[serde(default)]
    pub clear_parent: bool,
    #[serde(
        default,
        deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime"
    )]
    pub due_date: Option<NaiveDateTime>,
    #[serde(default)]
    pub clear_due_date: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoveTicketPayload {
    pub column_id: String,
    pub position: i32,
}
