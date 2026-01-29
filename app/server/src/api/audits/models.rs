use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

use crate::api::users::User;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, Insertable, ToSchema)]
#[diesel(table_name = crate::schema::audits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Audit {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub id: String,
    #[schema(example = "Security Audit Q1 2026")]
    pub name: String,
    #[schema(example = "Comprehensive security assessment")]
    pub description: Option<String>,
    pub team_id: String,
    pub company_id: String,
    pub end_date: Option<NaiveDateTime>,
    pub is_archived: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Audit {
    #[must_use]
    pub fn new(
        name: String,
        description: Option<String>,
        team_id: String,
        company_id: String,
        end_date: Option<NaiveDateTime>,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: Ulid::new().to_string(),
            name,
            description,
            team_id,
            company_id,
            end_date,
            is_archived: false,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn should_auto_archive(&self) -> bool {
        if self.is_archived {
            return false;
        }
        if let Some(end_date) = self.end_date {
            let now = chrono::Utc::now().naive_utc();
            return now > end_date;
        }
        false
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Selectable, Associations)]
#[diesel(table_name = crate::schema::audits_clients)]
#[diesel(belongs_to(Audit, foreign_key = audit_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(audit_id, user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuditClient {
    pub audit_id: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
}

impl AuditClient {
    #[must_use]
    pub fn new(audit_id: String, user_id: String) -> Self {
        Self {
            audit_id,
            user_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

// Request payloads
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuditPayload {
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "Security Audit Q1 2026")]
    pub name: String,

    #[schema(example = "Comprehensive security assessment")]
    pub description: Option<String>,

    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub team_id: String,

    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub company_id: String,

    #[serde(deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime")]
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuditPayload {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,

    #[serde(deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime")]
    pub end_date: Option<NaiveDateTime>,

    /// Set to true to remove the `end_date` (set to NULL)
    #[serde(default)]
    pub clear_end_date: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub team_id: String,
    pub company_id: String,
    pub clients: Vec<ClientUser>,
    pub end_date: Option<String>,
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClientUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<User> for ClientUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

impl AuditResponse {
    #[must_use]
    pub fn from_audit_and_clients(audit: Audit, clients: Vec<User>) -> Self {
        Self {
            id: audit.id,
            name: audit.name,
            description: audit.description,
            team_id: audit.team_id,
            company_id: audit.company_id,
            clients: clients.into_iter().map(ClientUser::from).collect(),
            end_date: audit.end_date.map(|d| d.to_string()),
            is_archived: audit.is_archived,
            created_at: audit.created_at.to_string(),
            updated_at: audit.updated_at.to_string(),
        }
    }
}
