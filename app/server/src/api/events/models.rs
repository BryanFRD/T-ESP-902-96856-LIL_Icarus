use chrono::NaiveDateTime;
use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{Output, ToSql},
    sql_types::Text,
    Identifiable, Insertable, Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, AsExpression, FromSqlRow, PartialEq, ToSchema)]
#[diesel(sql_type = Text)]
#[schema(example = "Create")]
pub enum EventAction {
    Read,
    Create,
    Update,
    Delete,
    Login,
    Logout,
    Register,
    PasswordReset,
    EmailConfirm,
    InvitationCreate,
    InvitationConsume,
    InvitationCancel,
    AvatarUpload,
    AvatarDelete,
}

impl<DB: Backend> FromSql<Text, DB> for EventAction
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "read" => Ok(Self::Read),
            "create" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "login" => Ok(Self::Login),
            "logout" => Ok(Self::Logout),
            "register" => Ok(Self::Register),
            "password_reset" => Ok(Self::PasswordReset),
            "email_confirm" => Ok(Self::EmailConfirm),
            "invitation_create" => Ok(Self::InvitationCreate),
            "invitation_consume" => Ok(Self::InvitationConsume),
            "invitation_cancel" => Ok(Self::InvitationCancel),
            "avatar_upload" => Ok(Self::AvatarUpload),
            "avatar_delete" => Ok(Self::AvatarDelete),
            _ => Err("Unrecognized event action".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for EventAction
where
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
        match self {
            Self::Read => "read".to_sql(out),
            Self::Create => "create".to_sql(out),
            Self::Update => "update".to_sql(out),
            Self::Delete => "delete".to_sql(out),
            Self::Login => "login".to_sql(out),
            Self::Logout => "logout".to_sql(out),
            Self::Register => "register".to_sql(out),
            Self::PasswordReset => "password_reset".to_sql(out),
            Self::EmailConfirm => "email_confirm".to_sql(out),
            Self::InvitationCreate => "invitation_create".to_sql(out),
            Self::InvitationConsume => "invitation_consume".to_sql(out),
            Self::InvitationCancel => "invitation_cancel".to_sql(out),
            Self::AvatarUpload => "avatar_upload".to_sql(out),
            Self::AvatarDelete => "avatar_delete".to_sql(out),
        }
    }
}

impl Display for EventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Create => write!(f, "create"),
            Self::Update => write!(f, "update"),
            Self::Delete => write!(f, "delete"),
            Self::Login => write!(f, "login"),
            Self::Logout => write!(f, "logout"),
            Self::Register => write!(f, "register"),
            Self::PasswordReset => write!(f, "password_reset"),
            Self::EmailConfirm => write!(f, "email_confirm"),
            Self::InvitationCreate => write!(f, "invitation_create"),
            Self::InvitationConsume => write!(f, "invitation_consume"),
            Self::InvitationCancel => write!(f, "invitation_cancel"),
            Self::AvatarUpload => write!(f, "avatar_upload"),
            Self::AvatarDelete => write!(f, "avatar_delete"),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, AsExpression, FromSqlRow, PartialEq, ToSchema)]
#[diesel(sql_type = Text)]
#[schema(example = "User")]
pub enum ResourceType {
    User,
    Team,
    TeamMember,
    Company,
    Invitation,
    Auth,
    Avatar,
}

impl<DB: Backend> FromSql<Text, DB> for ResourceType
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "user" => Ok(Self::User),
            "team" => Ok(Self::Team),
            "team_member" => Ok(Self::TeamMember),
            "company" => Ok(Self::Company),
            "invitation" => Ok(Self::Invitation),
            "auth" => Ok(Self::Auth),
            "avatar" => Ok(Self::Avatar),
            _ => Err("Unrecognized resource type".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for ResourceType
where
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
        match self {
            Self::User => "user".to_sql(out),
            Self::Team => "team".to_sql(out),
            Self::TeamMember => "team_member".to_sql(out),
            Self::Company => "company".to_sql(out),
            Self::Invitation => "invitation".to_sql(out),
            Self::Auth => "auth".to_sql(out),
            Self::Avatar => "avatar".to_sql(out),
        }
    }
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Team => write!(f, "team"),
            Self::TeamMember => write!(f, "team_member"),
            Self::Company => write!(f, "company"),
            Self::Invitation => write!(f, "invitation"),
            Self::Auth => write!(f, "auth"),
            Self::Avatar => write!(f, "avatar"),
        }
    }
}


#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, ToSchema,
)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub user_id: Option<String>,
    pub action: EventAction,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub request_method: String,
    pub request_path: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub metadata_nonce: Option<Vec<u8>>,
    pub changes_nonce: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::events)]
pub struct NewEvent {
    pub id: String,
    pub user_id: Option<String>,
    pub action: EventAction,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub request_method: String,
    pub request_path: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub metadata_nonce: Option<Vec<u8>>,
    pub changes_nonce: Option<Vec<u8>>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl NewEvent {
    #[must_use]
    pub fn new(
        user_id: Option<String>,
        action: EventAction,
        resource_type: ResourceType,
        resource_id: Option<String>,
        request_method: String,
        request_path: String,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        let expires_at = now + chrono::Duration::days(365);

        Self {
            id: ulid::Ulid::new().to_string(),
            user_id,
            action,
            resource_type,
            resource_id,
            request_method,
            request_path,
            ip_address: None,
            user_agent: None,
            metadata: None,
            changes: None,
            metadata_nonce: None,
            changes_nonce: None,
            created_at: now,
            expires_at,
        }
    }

    #[must_use]
    pub fn with_ip_address(mut self, ip: Option<String>) -> Self {
        self.ip_address = ip;
        self
    }

    #[must_use]
    pub fn with_user_agent(mut self, ua: Option<String>) -> Self {
        self.user_agent = ua;
        self
    }

    #[must_use]
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    #[must_use]
    pub fn with_changes(mut self, changes: EventChanges) -> Self {
        self.changes = serde_json::to_value(changes).ok();
        self
    }

    #[must_use]
    pub fn with_metadata_nonce(mut self, nonce: Option<Vec<u8>>) -> Self {
        self.metadata_nonce = nonce;
        self
    }

    #[must_use]
    pub fn with_changes_nonce(mut self, nonce: Option<Vec<u8>>) -> Self {
        self.changes_nonce = nonce;
        self
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventChanges {
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
}

impl EventChanges {
    #[must_use]
    pub fn new(before: Option<serde_json::Value>, after: Option<serde_json::Value>) -> Self {
        Self { before, after }
    }
}


#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventResponse {
    pub id: String,
    pub user_id: Option<String>,
    pub action: EventAction,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub request_method: String,
    pub request_path: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub created_at: String,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            user_id: event.user_id,
            action: event.action,
            resource_type: event.resource_type,
            resource_id: event.resource_id,
            request_method: event.request_method,
            request_path: event.request_path,
            ip_address: event.ip_address,
            user_agent: event.user_agent,
            metadata: event.metadata,
            changes: event.changes,
            created_at: event.created_at.to_string(),
        }
    }
}


#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventFilters {
    pub user_id: Option<String>,
    pub resource_type: Option<ResourceType>,
    pub resource_id: Option<String>,
    pub action: Option<EventAction>,
    pub date_from: Option<NaiveDateTime>,
    pub date_to: Option<NaiveDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    #[serde(default = "default_anonymized")]
    pub anonymized: bool,
}

fn default_anonymized() -> bool {
    true
}
