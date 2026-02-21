use shared::multiplexing::StreamId;
use socketioxide::SocketIo;
use tracing::error;

use super::types::{AgentOutputPayload, AgentStatusPayload, AgentStreamClosedPayload};

/// Emit agent output data to the audit room.
///
/// Called when the server receives `StreamData` from an agent's TCP connection.
pub async fn emit_agent_output(
    io: &SocketIo,
    audit_id: &str,
    agent_id: &str,
    stream_id: StreamId,
    data: &[u8],
) {
    use base64::{Engine, engine::general_purpose::STANDARD};

    let room_id = format!("audit:{audit_id}");
    let payload = AgentOutputPayload {
        agent_id: agent_id.to_string(),
        stream_id,
        data: STANDARD.encode(data),
    };
    if let Err(e) = io
        .of("/v1/ws")
        .expect("namespace /v1/ws not found")
        .to(room_id)
        .emit("agent:output", &payload)
        .await
    {
        error!(audit_id = %audit_id, agent_id = %agent_id, error = %e, "Failed to emit agent:output");
    }
}

/// Emit agent online/offline status to the audit room.
pub async fn emit_agent_status(io: &SocketIo, audit_id: &str, agent_id: &str, online: bool) {
    let room_id = format!("audit:{audit_id}");
    let payload = AgentStatusPayload {
        agent_id: agent_id.to_string(),
        online,
    };
    if let Err(e) = io
        .of("/v1/ws")
        .expect("namespace /v1/ws not found")
        .to(room_id)
        .emit("agent:status", &payload)
        .await
    {
        error!(audit_id = %audit_id, agent_id = %agent_id, error = %e, "Failed to emit agent:status");
    }
}

/// Emit agent stream closed to the audit room.
pub async fn emit_agent_stream_closed(
    io: &SocketIo,
    audit_id: &str,
    agent_id: &str,
    stream_id: StreamId,
) {
    let room_id = format!("audit:{audit_id}");
    let payload = AgentStreamClosedPayload {
        agent_id: agent_id.to_string(),
        stream_id,
    };
    if let Err(e) = io
        .of("/v1/ws")
        .expect("namespace /v1/ws not found")
        .to(room_id)
        .emit("agent:stream_closed", &payload)
        .await
    {
        error!(audit_id = %audit_id, agent_id = %agent_id, error = %e, "Failed to emit agent:stream_closed");
    }
}
