use serde::Serialize;

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
