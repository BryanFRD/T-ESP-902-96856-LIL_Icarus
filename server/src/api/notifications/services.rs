use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;
use socketioxide::SocketIo;
use tracing::{error, info};
use ulid::Ulid;

use super::{
    models::{NOTIFICATION_TYPE_AUDIT_INVITATION, NOTIFICATION_TYPE_TEAM_INVITATION},
    NewNotification, Notification, NotificationError,
};
use crate::schema::notifications;
use crate::state::GlobalState;

/// Create a new notification in the database
pub fn create_notification(
    state: &GlobalState,
    user_id: &str,
    notification_type: &str,
    metadata: Option<serde_json::Value>,
) -> Result<Notification, NotificationError> {
    let mut conn = state.get_db_conn()?;

    let now = Utc::now().naive_utc();
    let new_notification = NewNotification {
        id: Ulid::new().to_string(),
        user_id: user_id.to_string(),
        type_: notification_type.to_string(),
        is_read: false,
        metadata,
        created_at: now,
        updated_at: now,
    };

    let notification = diesel::insert_into(notifications::table)
        .values(&new_notification)
        .get_result::<Notification>(&mut conn)?;

    info!(
        "Created notification {} for user {} of type {}",
        notification.id, user_id, notification_type
    );

    Ok(notification)
}

/// Get all unread notifications for a user
pub fn get_unread_notifications(
    state: &GlobalState,
    user_id: &str,
) -> Result<Vec<Notification>, NotificationError> {
    let mut conn = state.get_db_conn()?;

    let results = notifications::table
        .filter(notifications::user_id.eq(user_id))
        .filter(notifications::is_read.eq(false))
        .order(notifications::created_at.desc())
        .load::<Notification>(&mut conn)?;

    Ok(results)
}

/// Mark a notification as read
pub fn mark_as_read(
    state: &GlobalState,
    notification_id: &str,
    user_id: &str,
) -> Result<(), NotificationError> {
    let mut conn = state.get_db_conn()?;

    // First check if notification exists and belongs to user
    let notification = notifications::table
        .find(notification_id)
        .first::<Notification>(&mut conn)
        .optional()?;

    match notification {
        None => return Err(NotificationError::NotFound),
        Some(notif) if notif.user_id != user_id => return Err(NotificationError::Unauthorized),
        _ => {}
    }

    // Update is_read to true
    diesel::update(notifications::table.find(notification_id))
        .set(notifications::is_read.eq(true))
        .execute(&mut conn)?;

    info!(
        "Marked notification {} as read for user {}",
        notification_id, user_id
    );

    Ok(())
}

/// Emit a new notification via Socket.IO to all connected clients of a user
pub fn emit_new_notification(io: &SocketIo, user_id: &str, notification: &Notification) {
    // Emit to the user's room (all their connected sockets)
    if let Err(e) = io
        .of("/")
        .unwrap()
        .to(user_id)
        .emit("notification:new", notification)
    {
        error!("Failed to emit notification to user {}: {}", user_id, e);
    } else {
        info!(
            "Emitted notification {} to user {}",
            notification.id, user_id
        );
    }
}

/// Helper to create team invitation notification
pub fn create_team_invitation_notification(
    state: &GlobalState,
    io: &SocketIo,
    user_id: &str,
    team_id: &str,
    team_name: &str,
    inviter_name: &str,
) -> Result<Notification, NotificationError> {
    let metadata = json!({
        "team_id": team_id,
        "team_name": team_name,
        "inviter_name": inviter_name,
    });

    let notification = create_notification(
        state,
        user_id,
        NOTIFICATION_TYPE_TEAM_INVITATION,
        Some(metadata),
    )?;

    emit_new_notification(io, user_id, &notification);

    Ok(notification)
}

/// Helper to create audit invitation notification
pub fn create_audit_invitation_notification(
    state: &GlobalState,
    io: &SocketIo,
    user_id: &str,
    audit_id: &str,
    audit_name: &str,
    team_id: &str,
    team_name: &str,
    inviter_name: &str,
) -> Result<Notification, NotificationError> {
    let metadata = json!({
        "audit_id": audit_id,
        "audit_name": audit_name,
        "team_id": team_id,
        "team_name": team_name,
        "inviter_name": inviter_name,
    });

    let notification = create_notification(
        state,
        user_id,
        NOTIFICATION_TYPE_AUDIT_INVITATION,
        Some(metadata),
    )?;

    emit_new_notification(io, user_id, &notification);

    Ok(notification)
}

pub fn get_routes(_state: &GlobalState) -> axum::Router {
    // Future REST endpoints can be added here
    axum::Router::new()
}
