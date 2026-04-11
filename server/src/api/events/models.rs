use chrono::NaiveDateTime;
use diesel::{
    Identifiable, Insertable, Queryable, Selectable,
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{Output, ToSql},
    sql_types::Text,
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
    AgentConnect,
    AgentDisconnect,
    AgentCommand,
    AgentShutdown,
    AgentDownload,
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
            "agent_connect" => Ok(Self::AgentConnect),
            "agent_disconnect" => Ok(Self::AgentDisconnect),
            "agent_command" => Ok(Self::AgentCommand),
            "agent_shutdown" => Ok(Self::AgentShutdown),
            "agent_download" => Ok(Self::AgentDownload),
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
            Self::AgentConnect => "agent_connect".to_sql(out),
            Self::AgentDisconnect => "agent_disconnect".to_sql(out),
            Self::AgentCommand => "agent_command".to_sql(out),
            Self::AgentShutdown => "agent_shutdown".to_sql(out),
            Self::AgentDownload => "agent_download".to_sql(out),
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
            Self::AgentConnect => write!(f, "agent_connect"),
            Self::AgentDisconnect => write!(f, "agent_disconnect"),
            Self::AgentCommand => write!(f, "agent_command"),
            Self::AgentShutdown => write!(f, "agent_shutdown"),
            Self::AgentDownload => write!(f, "agent_download"),
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
    Audit,
    AuditClient,
    AuditInvitation,
    Ticket,
    TicketComment,
    TicketAttachment,
    KanbanColumn,
    TicketLabel,
    Agent,
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
            "audit" => Ok(Self::Audit),
            "audit_client" => Ok(Self::AuditClient),
            "audit_invitation" => Ok(Self::AuditInvitation),
            "ticket" => Ok(Self::Ticket),
            "ticket_comment" => Ok(Self::TicketComment),
            "ticket_attachment" => Ok(Self::TicketAttachment),
            "kanban_column" => Ok(Self::KanbanColumn),
            "ticket_label" => Ok(Self::TicketLabel),
            "agent" => Ok(Self::Agent),
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
            Self::Audit => "audit".to_sql(out),
            Self::AuditClient => "audit_client".to_sql(out),
            Self::AuditInvitation => "audit_invitation".to_sql(out),
            Self::Ticket => "ticket".to_sql(out),
            Self::TicketComment => "ticket_comment".to_sql(out),
            Self::TicketAttachment => "ticket_attachment".to_sql(out),
            Self::KanbanColumn => "kanban_column".to_sql(out),
            Self::TicketLabel => "ticket_label".to_sql(out),
            Self::Agent => "agent".to_sql(out),
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
            Self::Audit => write!(f, "audit"),
            Self::AuditClient => write!(f, "audit_client"),
            Self::AuditInvitation => write!(f, "audit_invitation"),
            Self::Ticket => write!(f, "ticket"),
            Self::TicketComment => write!(f, "ticket_comment"),
            Self::TicketAttachment => write!(f, "ticket_attachment"),
            Self::KanbanColumn => write!(f, "kanban_column"),
            Self::TicketLabel => write!(f, "ticket_label"),
            Self::Agent => write!(f, "agent"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, ToSchema)]
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

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedEventLogResponse {
    pub data: Vec<EventLogResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventLogResponse {
    pub id: String,
    pub user_id: Option<String>,
    pub user: Option<EventLogUser>,
    pub action: EventAction,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub request_method: String,
    pub request_path: String,
    pub metadata: Option<serde_json::Value>,
    pub changes: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventLogUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogFilters {
    pub include_user: Option<bool>,
    pub action: Option<EventAction>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
