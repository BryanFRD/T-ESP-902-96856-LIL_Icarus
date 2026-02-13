use serde::Serialize;
use shared::multiplexing::StreamId;

/// Standard notification payload for user-targeted events.
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct UserNotificationPayload {
    pub title: String,
    pub message: String,
    /// One of: `"info"`, `"warning"`, `"success"`, `"error"`.
    pub notification_type: String,
}

/// Payload emitted when a user receives a team or audit invitation.
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct InvitationReceivedPayload {
    pub invitation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_name: Option<String>,
    pub invited_by: String,
}

/// Payload emitted when the server relays output data from an agent stream.
#[derive(Debug, Clone, Serialize)]
pub struct AgentOutputPayload {
    pub agent_id: String,
    pub stream_id: StreamId,
    /// Base64-encoded binary data from the agent.
    pub data: String,
}

/// Payload emitted when an agent comes online or goes offline.
#[derive(Debug, Clone, Serialize)]
pub struct AgentStatusPayload {
    pub agent_id: String,
    pub online: bool,
}

/// Payload emitted when an agent stream is closed.
#[derive(Debug, Clone, Serialize)]
pub struct AgentStreamClosedPayload {
    pub agent_id: String,
    pub stream_id: StreamId,
}
