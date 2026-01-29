use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, Selectable, ToSchema,
)]
#[diesel(table_name = crate::schema::teams_invitations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct TeamInvitation {
    pub id: String,
    pub team_id: String,
    pub token: String,
    pub max_uses: i32,
    pub used_count: i32,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub created_by: String,
    pub is_canceled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TeamInvitation {
    pub fn new(
        team_id: String,
        created_by: String,
        max_uses: i32,
        expires_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        TeamInvitation {
            id: ulid::Ulid::new().to_string(),
            team_id,
            token: ulid::Ulid::new().to_string(),
            max_uses,
            used_count: 0,
            expires_at,
            created_by,
            is_canceled: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationPayload {
    pub max_uses: i32,
    #[serde(deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime")]
    pub expires_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationReturn {
    pub id: String,
    pub team_id: String,
    pub token: String,
    pub max_uses: i32,
    pub used_count: i32,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub is_canceled: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<TeamInvitation> for InvitationReturn {
    fn from(i: TeamInvitation) -> Self {
        InvitationReturn {
            id: i.id,
            team_id: i.team_id,
            token: i.token,
            max_uses: i.max_uses,
            used_count: i.used_count,
            expires_at: i.expires_at,
            is_canceled: i.is_canceled,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}
