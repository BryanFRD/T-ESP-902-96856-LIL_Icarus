use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::audit_agents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuditAgent {
    pub id: String,
    pub audit_id: String,
    pub name: String,
    pub token_hash: String,
    pub is_revoked: bool,
    pub last_connected_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl AuditAgent {
    #[must_use]
    pub fn new(audit_id: String, name: String, token_hash: String) -> Self {
        Self {
            id: Ulid::new().to_string(),
            audit_id,
            name,
            token_hash,
            is_revoked: false,
            last_connected_at: None,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentPayload {
    #[validate(length(min = 1, max = 255))]
    #[schema(example = "agent-pc-cible-1")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfoResponse {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub cpu_name: String,
    pub cpu_count: u32,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub uptime_secs: u64,
}

impl From<shared::packets::SystemInfo> for SystemInfoResponse {
    fn from(info: shared::packets::SystemInfo) -> Self {
        Self {
            hostname: info.hostname,
            os_name: info.os_name,
            os_version: info.os_version,
            cpu_name: info.cpu_name,
            cpu_count: info.cpu_count,
            total_memory_bytes: info.total_memory_bytes,
            used_memory_bytes: info.used_memory_bytes,
            uptime_secs: info.uptime_secs,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponse {
    pub id: String,
    pub audit_id: String,
    pub name: String,
    pub is_revoked: bool,
    pub is_online: bool,
    pub last_connected_at: Option<String>,
    pub last_heartbeat: Option<String>,
    pub system_info: Option<SystemInfoResponse>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AgentTokenResponse {
    pub id: String,
    pub audit_id: String,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum ShellType {
    Auto,
    Bash,
    Sh,
    Cmd,
    PowerShell,
}

impl From<ShellType> for shared::packets::ShellType {
    fn from(t: ShellType) -> Self {
        match t {
            ShellType::Auto => shared::packets::ShellType::Auto,
            ShellType::Bash => shared::packets::ShellType::Bash,
            ShellType::Sh => shared::packets::ShellType::Sh,
            ShellType::Cmd => shared::packets::ShellType::Cmd,
            ShellType::PowerShell => shared::packets::ShellType::PowerShell,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ShellRequestPayload {
    pub shell_type: ShellType,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ShellStreamResponse {
    pub stream_id: u32,
    pub message: String,
}

/// Action to perform on the keylogger
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum KeyloggerAction {
    Start,
    Stop,
}

/// Payload for keylogger start/stop requests
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyloggerRequestPayload {
    pub action: KeyloggerAction,
}

/// Response for keylogger requests
#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KeyloggerResponse {
    pub message: String,
}
