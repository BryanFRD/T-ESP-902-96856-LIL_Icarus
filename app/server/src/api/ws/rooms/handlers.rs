use serde::Deserialize;
use shared::packets::{AgentControl, Packet, StreamData, StreamOpen};
use socketioxide::extract::{Data, SocketRef, State};
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};

use crate::api::auth::models::JwtClaims;
use crate::api::events::{EventAction, NewEvent, ResourceType};
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
        if let Err(e) = validate_step_up_from_headers(headers, &state.audit_access_secret, &user_id)
        {
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

// --- Agent event payloads ---

/// Payload for the `agent:shutdown` event.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentShutdownPayload {
    pub audit_id: String,
    pub agent_id: String,
    pub reason: Option<String>,
}

/// Payload for the `agent:command` event.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentCommandPayload {
    pub audit_id: String,
    pub agent_id: String,
    pub command: String,
}

// --- Agent event handlers ---

/// Handler for the `agent:shutdown` event.
///
/// Sends an `AgentShutdown` packet to the connected agent and shuts down its connection.
pub async fn on_agent_shutdown(
    socket: SocketRef,
    Data(payload): Data<AgentShutdownPayload>,
    State(state): State<GlobalState>,
) {
    let Some(claims) = socket.extensions.get::<JwtClaims>() else {
        emit_error(&socket, "AUTH_FAILED", "Not authenticated");
        return;
    };

    let user_id = claims.sub.to_string();
    info!(
        user_id = %user_id,
        agent_id = %payload.agent_id,
        audit_id = %payload.audit_id,
        "agent:shutdown requested"
    );

    // Verify the user has permission (must be able to modify the audit)
    let can_modify = match crate::api::audits::get_audit(&state, &payload.audit_id) {
        Ok(audit) => {
            crate::api::audits::can_modify_audit(&state, &audit, &user_id, claims.is_admin())
                .unwrap_or(false)
        }
        Err(_) => false,
    };

    if !can_modify {
        emit_error(
            &socket,
            "PERMISSION_DENIED",
            "Not authorized to shutdown this agent",
        );
        return;
    }

    // Extract writer + shared_secret from handle, then drop the lock before awaiting
    let agent_writer_info = {
        let Ok(registry) = state.agent_registry.lock() else {
            emit_error(&socket, "INTERNAL_ERROR", "Failed to access agent registry");
            return;
        };

        let Some(handle) = registry.get(&payload.agent_id) else {
            emit_error(&socket, "AGENT_NOT_FOUND", "Agent is not connected");
            return;
        };

        if handle.audit_id != payload.audit_id {
            emit_error(&socket, "AGENT_NOT_FOUND", "Agent is not connected");
            return;
        }

        let writer = handle.writer.clone();
        let secret = handle.shared_secret;
        handle.shutdown();
        (writer, secret)
    };

    // Now send the shutdown packet without holding the registry lock
    let reason = payload
        .reason
        .unwrap_or_else(|| "Shutdown requested by user".to_string());
    let shutdown_packet = AgentControl::shutdown(reason);
    if let Ok(data) = shutdown_packet.serialize() {
        let (encrypted, nonce) = match shared::encryption::encrypt(&agent_writer_info.1, &data) {
            Ok(v) => v,
            Err(e) => {
                warn!(agent_id = %payload.agent_id, error = %e, "Failed to encrypt shutdown packet");
                return;
            }
        };

        #[allow(clippy::cast_possible_truncation)]
        let len = encrypted.len() as u32;
        let total_size = 4 + nonce.len() + encrypted.len();
        let mut buf = vec![0u8; total_size];
        buf[0..4].copy_from_slice(&len.to_be_bytes());
        buf[4..16].copy_from_slice(&nonce);
        buf[16..].copy_from_slice(&encrypted);

        let mut writer = agent_writer_info.0.lock().await;
        if let Err(e) = writer.write_all(&buf).await {
            warn!(agent_id = %payload.agent_id, error = %e, "Failed to send shutdown packet");
        }
        let _ = writer.flush().await;
    }

    info!(
        user_id = %user_id,
        agent_id = %payload.agent_id,
        "Agent shutdown signal sent"
    );

    // Log agent shutdown event
    let shutdown_event = NewEvent::new(
        Some(user_id.clone()),
        EventAction::AgentShutdown,
        ResourceType::Agent,
        Some(format!(
            "agent_id:{},audit_id:{}",
            payload.agent_id, payload.audit_id
        )),
        "WS".to_string(),
        "/ws/agent:shutdown".to_string(),
    )
    .with_metadata(serde_json::json!({
        "agent_id": payload.agent_id,
        "audit_id": payload.audit_id,
    }));

    if let Err(e) = crate::api::events::services::log_event_sync(&state, &shutdown_event) {
        error!("Failed to log agent shutdown event: {e}");
    }
}

/// Handler for the `agent:command` event.
///
/// Opens a multiplexed stream to the agent and sends command data through it.
#[allow(clippy::too_many_lines)]
pub async fn on_agent_command(
    socket: SocketRef,
    Data(payload): Data<AgentCommandPayload>,
    State(state): State<GlobalState>,
) {
    let Some(claims) = socket.extensions.get::<JwtClaims>() else {
        emit_error(&socket, "AUTH_FAILED", "Not authenticated");
        return;
    };

    let user_id = claims.sub.to_string();
    info!(
        user_id = %user_id,
        agent_id = %payload.agent_id,
        audit_id = %payload.audit_id,
        command = %payload.command,
        "agent:command requested"
    );

    // Verify the user has permission
    let can_modify = match crate::api::audits::get_audit(&state, &payload.audit_id) {
        Ok(audit) => {
            crate::api::audits::can_modify_audit(&state, &audit, &user_id, claims.is_admin())
                .unwrap_or(false)
        }
        Err(_) => false,
    };

    if !can_modify {
        emit_error(
            &socket,
            "PERMISSION_DENIED",
            "Not authorized to send commands to this agent",
        );
        return;
    }

    // Extract writer + shared_secret from handle, then drop the lock before awaiting
    let agent_writer_info = {
        let Ok(registry) = state.agent_registry.lock() else {
            emit_error(&socket, "INTERNAL_ERROR", "Failed to access agent registry");
            return;
        };

        let Some(handle) = registry.get(&payload.agent_id) else {
            emit_error(&socket, "AGENT_NOT_FOUND", "Agent is not connected");
            return;
        };

        if handle.audit_id != payload.audit_id {
            emit_error(&socket, "AGENT_NOT_FOUND", "Agent is not connected");
            return;
        }

        (handle.writer.clone(), handle.shared_secret)
    };

    let (writer, shared_secret) = agent_writer_info;

    // Open a new stream: use a simple incrementing stream ID based on timestamp
    let stream_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(1, |d| (d.as_millis() & 0xFFFF_FFFF) as u32)
        | shared::multiplexing::MIN_DATA_STREAM_ID;

    // Helper to encrypt and send a packet
    let send = |data: &[u8]| -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let (encrypted, nonce) = shared::encryption::encrypt(&shared_secret, data)?;
        #[allow(clippy::cast_possible_truncation)]
        let len = encrypted.len() as u32;
        let total_size = 4 + nonce.len() + encrypted.len();
        let mut buf = vec![0u8; total_size];
        buf[0..4].copy_from_slice(&len.to_be_bytes());
        buf[4..16].copy_from_slice(&nonce);
        buf[16..].copy_from_slice(&encrypted);
        Ok(buf)
    };

    // Send StreamOpen
    let open_packet = StreamOpen { stream_id };
    let open_buf = match open_packet.serialize().and_then(|d| {
        send(&d).map_err(|e| shared::packets::PacketError::EncodingError(e.to_string()))
    }) {
        Ok(buf) => buf,
        Err(e) => {
            emit_error(
                &socket,
                "INTERNAL_ERROR",
                &format!("Failed to prepare stream open: {e}"),
            );
            return;
        }
    };

    // Send StreamData
    let data_packet = StreamData {
        stream_id,
        data: payload.command.into_bytes(),
    };
    let data_buf = match data_packet.serialize().and_then(|d| {
        send(&d).map_err(|e| shared::packets::PacketError::EncodingError(e.to_string()))
    }) {
        Ok(buf) => buf,
        Err(e) => {
            emit_error(
                &socket,
                "INTERNAL_ERROR",
                &format!("Failed to prepare command: {e}"),
            );
            return;
        }
    };

    // Send both packets (lock the writer once)
    let mut w = writer.lock().await;
    if let Err(e) = w.write_all(&open_buf).await {
        emit_error(
            &socket,
            "SEND_FAILED",
            &format!("Failed to open stream: {e}"),
        );
        return;
    }
    if let Err(e) = w.write_all(&data_buf).await {
        emit_error(
            &socket,
            "SEND_FAILED",
            &format!("Failed to send command: {e}"),
        );
        return;
    }
    if let Err(e) = w.flush().await {
        emit_error(&socket, "SEND_FAILED", &format!("Failed to flush: {e}"));
        return;
    }
    drop(w);

    info!(
        user_id = %user_id,
        agent_id = %payload.agent_id,
        stream_id = stream_id,
        "Command sent to agent"
    );

    // Log agent command event
    let command_event = NewEvent::new(
        Some(user_id),
        EventAction::AgentCommand,
        ResourceType::Agent,
        Some(format!(
            "agent_id:{},audit_id:{}",
            payload.agent_id, payload.audit_id
        )),
        "WS".to_string(),
        "/ws/agent:command".to_string(),
    )
    .with_metadata(serde_json::json!({
        "agent_id": payload.agent_id,
        "audit_id": payload.audit_id,
        "stream_id": stream_id,
    }));

    if let Err(e) = crate::api::events::services::log_event_sync(&state, &command_event) {
        error!("Failed to log agent command event: {e}");
    }
}
