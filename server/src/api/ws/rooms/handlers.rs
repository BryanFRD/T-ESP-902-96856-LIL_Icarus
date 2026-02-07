use socketioxide::extract::{Data, SocketRef, State};
use tracing::{error, info, warn};

use crate::api::auth::models::JwtClaims;
use crate::api::ws::error::WsErrorResponse;
use crate::state::GlobalState;

use super::permissions::{can_join_room, validate_step_up_from_headers};
use super::types::{Room, RoomPayload};

/// Response sent to the client after successfully joining a room.
#[derive(serde::Serialize)]
struct RoomJoinedResponse {
    room: String,
}

/// Response sent to the client after successfully leaving a room.
#[derive(serde::Serialize)]
struct RoomLeftResponse {
    room: String,
}

/// Handler for the `room:join` event.
///
/// The client sends a [`RoomPayload`] and the server validates permissions before
/// adding the socket to the corresponding Socket.IO room.
pub async fn on_room_join(
    socket: SocketRef,
    Data(payload): Data<RoomPayload>,
    State(state): State<GlobalState>,
) {
    let Some(claims) = socket.extensions.get::<JwtClaims>() else {
        emit_error(&socket, "AUTH_FAILED", "Not authenticated");
        return;
    };

    let user_id = claims.sub.to_string();
    let room = Room::from_payload(&payload, &user_id);

    // For audit rooms, validate step-up authentication first
    if let Room::Audit(_) = &room {
        let headers = &socket.req_parts().headers;
        if let Err(e) = validate_step_up_from_headers(
            headers,
            &state.audit_access_secret,
            &user_id,
        ) {
            warn!(
                user_id = %user_id,
                room = %room.display_name(),
                error = %e,
                "Step-up validation failed for audit room join"
            );
            emit_ws_error(&socket, &e);
            return;
        }
    }

    // Check room-level permissions (team membership, audit client, etc.)
    if let Err(e) = can_join_room(&state, &user_id, &room, claims.is_admin()) {
        warn!(
            user_id = %user_id,
            room = %room.display_name(),
            error = %e,
            "Room access denied"
        );
        emit_ws_error(&socket, &e);
        return;
    }

    let room_id = room.room_id();
    let display = room.display_name();

    socket.join(room_id.clone());

    info!(
        user_id = %user_id,
        socket_id = %socket.id,
        room = %room_id,
        "Socket joined room"
    );

    if let Err(e) = socket.emit("room:joined", &RoomJoinedResponse { room: display }) {
        error!("Failed to emit room:joined: {e}");
    }
}

/// Handler for the `room:leave` event.
pub async fn on_room_leave(socket: SocketRef, Data(payload): Data<RoomPayload>) {
    let Some(claims) = socket.extensions.get::<JwtClaims>() else {
        emit_error(&socket, "AUTH_FAILED", "Not authenticated");
        return;
    };

    let user_id = claims.sub.to_string();
    let room = Room::from_payload(&payload, &user_id);
    let room_id = room.room_id();
    let display = room.display_name();

    socket.leave(room_id.clone());

    info!(
        user_id = %user_id,
        socket_id = %socket.id,
        room = %room_id,
        "Socket left room"
    );

    if let Err(e) = socket.emit("room:left", &RoomLeftResponse { room: display }) {
        error!("Failed to emit room:left: {e}");
    }
}

/// Emit a `room:error` event from a [`WsError`](crate::api::ws::error::WsError).
fn emit_ws_error(socket: &SocketRef, error: &crate::api::ws::error::WsError) {
    let response = error.to_response();
    if let Err(e) = socket.emit("room:error", &response) {
        tracing::error!("Failed to emit room:error: {e}");
    }
}

/// Emit a `room:error` event with an ad-hoc code and message.
fn emit_error(socket: &SocketRef, code: &str, message: &str) {
    let response = WsErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    };
    if let Err(e) = socket.emit("room:error", &response) {
        tracing::error!("Failed to emit room:error: {e}");
    }
}
