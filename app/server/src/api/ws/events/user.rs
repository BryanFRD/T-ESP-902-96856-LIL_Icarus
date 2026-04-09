use socketioxide::SocketIo;
use tracing::error;

use super::types::{InvitationReceivedPayload, UserNotificationPayload};

/// Emit a generic notification to a specific user.
///
/// The event is sent to the `user:{user_id}` room so that every connected
/// socket for that user receives it.
#[allow(dead_code)]
pub async fn emit_notification(io: &SocketIo, user_id: &str, payload: &UserNotificationPayload) {
    let room_id = format!("user:{user_id}");
    if let Err(e) = io
        .of("/")
        .expect("namespace / not found")
        .to(room_id)
        .emit("user:notification", payload)
        .await
    {
        error!(user_id = %user_id, error = %e, "Failed to emit user:notification");
    }
}

/// Emit an invitation-received event to a specific user.
#[allow(dead_code)]
pub async fn emit_invitation_received(
    io: &SocketIo,
    user_id: &str,
    payload: &InvitationReceivedPayload,
) {
    let room_id = format!("user:{user_id}");
    if let Err(e) = io
        .of("/")
        .expect("namespace / not found")
        .to(room_id)
        .emit("user:invitation_received", payload)
        .await
    {
        error!(user_id = %user_id, error = %e, "Failed to emit user:invitation_received");
    }
}
