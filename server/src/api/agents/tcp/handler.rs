use std::net::SocketAddr;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared::{
    encryption::{decrypt, encrypt},
    packets::{
        AgentAuthResponse, AgentControl, EncryptionRequest, EncryptionResponse, Packet, Packets,
        SystemInfo, from_packet_bytes,
    },
};
use socketioxide::SocketIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::api::events::{EventAction, NewEvent, ResourceType};
use crate::state::{AgentHandle, GlobalState};

/// Handle a single agent TCP connection: ECDH handshake, then token auth, then multiplex loop.
///
/// # Errors
/// Returns an error if the handshake or auth fails.
pub async fn handle_agent_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    state: GlobalState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("New agent connection from {addr}");

    // --- Phase 1: ECDH Handshake (server side) ---
    let shared_secret = perform_server_handshake(&mut stream).await?;
    info!("ECDH handshake completed with {addr}");

    // --- Phase 2: Token Authentication ---
    let agent = authenticate_agent(&mut stream, &shared_secret, &state).await?;
    info!(
        "Agent '{}' (id={}) authenticated for audit {} from {addr}",
        agent.name, agent.id, agent.audit_id
    );

    // Log agent connection event
    let connect_event = NewEvent::new(
        None,
        EventAction::AgentConnect,
        ResourceType::Agent,
        Some(format!("agent_id:{},audit_id:{}", agent.id, agent.audit_id)),
        "TCP".to_string(),
        format!("/tcp/agents/{}", agent.id),
    )
    .with_ip_address(Some(addr.ip().to_string()))
    .with_metadata(serde_json::json!({
        "agent_name": agent.name,
        "agent_id": agent.id,
        "audit_id": agent.audit_id,
        "address": addr.to_string(),
    }));

    if let Err(e) = crate::api::events::services::log_event_sync(&state, &connect_event) {
        error!("Failed to log agent connect event: {e}");
    }

    // --- Phase 3: Split stream and register in agent registry ---
    let (read_half, write_half) = stream.into_split();
    let writer = Arc::new(Mutex::new(write_half));

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);
    let last_heartbeat: Arc<std::sync::Mutex<Option<DateTime<Utc>>>> =
        Arc::new(std::sync::Mutex::new(Some(Utc::now())));
    let latency_ms: Arc<std::sync::Mutex<Option<u64>>> =
        Arc::new(std::sync::Mutex::new(None));
    let shell_stream_id: Arc<std::sync::Mutex<Option<u32>>> =
        Arc::new(std::sync::Mutex::new(None));
    let system_info: Arc<std::sync::Mutex<Option<SystemInfo>>> =
        Arc::new(std::sync::Mutex::new(None));
    {
        let handle = AgentHandle {
            agent_id: agent.id.clone(),
            audit_id: agent.audit_id.clone(),
            addr,
            shutdown: shutdown_tx,
            last_heartbeat: last_heartbeat.clone(),
            latency_ms: latency_ms.clone(),
            shell_stream_id: shell_stream_id.clone(),
            system_info: system_info.clone(),
            writer: writer.clone(),
            shared_secret,
        };
        if let Ok(mut registry) = state.agent_registry.lock() {
            registry.insert(agent.id.clone(), handle);
        }
    }

    // Emit agent:status online via WebSocket
    if let Some(io) = state.io_opt() {
        crate::api::ws::events::agent::emit_agent_status(io, &agent.audit_id, &agent.id, true)
            .await;
    }

    // --- Phase 4: Multiplex loop ---
    let result = multiplex_loop(
        read_half,
        &writer,
        &shared_secret,
        &mut shutdown_rx,
        &last_heartbeat,
        &latency_ms,
        &shell_stream_id,
        &system_info,
        state.io_opt(),
        &agent.audit_id,
        &agent.id,
    )
    .await;

    // Cleanup: remove from registry
    if let Ok(mut registry) = state.agent_registry.lock() {
        registry.remove(&agent.id);
    }

    // Log agent disconnection event
    let disconnect_event = NewEvent::new(
        None,
        EventAction::AgentDisconnect,
        ResourceType::Agent,
        Some(format!("agent_id:{},audit_id:{}", agent.id, agent.audit_id)),
        "TCP".to_string(),
        format!("/tcp/agents/{}", agent.id),
    )
    .with_ip_address(Some(addr.ip().to_string()))
    .with_metadata(serde_json::json!({
        "agent_name": agent.name,
        "agent_id": agent.id,
        "audit_id": agent.audit_id,
    }));

    if let Err(e) = crate::api::events::services::log_event_sync(&state, &disconnect_event) {
        error!("Failed to log agent disconnect event: {e}");
    }

    // Emit agent:status offline via WebSocket
    if let Some(io) = state.io_opt() {
        crate::api::ws::events::agent::emit_agent_status(io, &agent.audit_id, &agent.id, false)
            .await;
    }

    info!("Agent '{}' disconnected from {addr}", agent.name);

    result
}

/// Perform server-side ECDH handshake.
///
/// 1. Read `EncryptionRequest` from agent (pubkey + `verify_token`)
/// 2. Generate server keypair, compute shared secret
/// 3. Encrypt `verify_token` with shared secret
/// 4. Send `EncryptionResponse` back
async fn perform_server_handshake(
    stream: &mut TcpStream,
) -> Result<[u8; 32], Box<dyn std::error::Error + Send + Sync>> {
    // Read EncryptionRequest
    let mut req_buf = vec![0u8; EncryptionRequest::PACKET_SIZE];
    stream.read_exact(&mut req_buf).await?;

    let Packets::EncryptionRequest(enc_req) = from_packet_bytes(&req_buf)? else {
        return Err("Expected EncryptionRequest packet".into());
    };

    // Generate server keypair
    let server_secret = EphemeralSecret::random_from_rng(aes_gcm::aead::OsRng);
    let server_public = PublicKey::from(&server_secret);

    // Compute shared secret
    let client_public = PublicKey::from(enc_req.key);
    let shared_secret = server_secret.diffie_hellman(&client_public).to_bytes();

    // Encrypt the verify token
    let verify_token_bytes = enc_req.verify_token.to_be_bytes();
    let (encrypted_token, nonce) = encrypt(&shared_secret, &verify_token_bytes)?;

    // Build response
    let nonce_array: [u8; 12] = nonce.try_into().map_err(|_| "Nonce length mismatch")?;
    let encrypted_token_array: [u8; 24] = encrypted_token
        .try_into()
        .map_err(|_| "Encrypted token length mismatch")?;

    let response =
        EncryptionResponse::new(server_public.to_bytes(), nonce_array, encrypted_token_array);
    let response_bytes = response.serialize()?;
    stream.write_all(&response_bytes).await?;
    stream.flush().await?;

    Ok(shared_secret)
}

/// Read an encrypted packet from the read half.
async fn read_encrypted_packet(
    reader: &mut OwnedReadHalf,
    shared_secret: &[u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut nonce_buf = [0u8; 12];
    reader.read_exact(&mut nonce_buf).await?;

    let mut encrypted_buf = vec![0u8; len];
    reader.read_exact(&mut encrypted_buf).await?;

    let decrypted = decrypt(shared_secret, &nonce_buf, &encrypted_buf)?;
    Ok(decrypted)
}

/// Send an encrypted packet via the shared writer.
async fn send_encrypted_packet(
    writer: &Arc<Mutex<OwnedWriteHalf>>,
    shared_secret: &[u8; 32],
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (encrypted, nonce) = encrypt(shared_secret, data)?;

    #[allow(clippy::cast_possible_truncation)]
    let len = encrypted.len() as u32;
    let total_size = 4 + nonce.len() + encrypted.len();
    let mut buf = vec![0u8; total_size];

    buf[0..4].copy_from_slice(&len.to_be_bytes());
    buf[4..16].copy_from_slice(&nonce);
    buf[16..].copy_from_slice(&encrypted);

    let mut w = writer.lock().await;
    w.write_all(&buf).await?;
    w.flush().await?;
    Ok(())
}

/// Send an encrypted packet during handshake (before stream split).
async fn send_encrypted_packet_stream(
    stream: &mut TcpStream,
    shared_secret: &[u8; 32],
    data: &[u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (encrypted, nonce) = encrypt(shared_secret, data)?;

    #[allow(clippy::cast_possible_truncation)]
    let len = encrypted.len() as u32;
    let total_size = 4 + nonce.len() + encrypted.len();
    let mut buf = vec![0u8; total_size];

    buf[0..4].copy_from_slice(&len.to_be_bytes());
    buf[4..16].copy_from_slice(&nonce);
    buf[16..].copy_from_slice(&encrypted);

    stream.write_all(&buf).await?;
    stream.flush().await?;
    Ok(())
}

/// Read an encrypted packet during handshake (before stream split).
async fn read_encrypted_packet_stream(
    stream: &mut TcpStream,
    shared_secret: &[u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    if len == 0 {
        return Ok(Vec::new());
    }

    let mut nonce_buf = [0u8; 12];
    stream.read_exact(&mut nonce_buf).await?;

    let mut encrypted_buf = vec![0u8; len];
    stream.read_exact(&mut encrypted_buf).await?;

    let decrypted = decrypt(shared_secret, &nonce_buf, &encrypted_buf)?;
    Ok(decrypted)
}

/// Authenticate the agent by reading an encrypted `AgentAuth` packet and verifying
/// the token against the database.
async fn authenticate_agent(
    stream: &mut TcpStream,
    shared_secret: &[u8; 32],
    state: &GlobalState,
) -> Result<super::super::AuditAgent, Box<dyn std::error::Error + Send + Sync>> {
    let packet_data = read_encrypted_packet_stream(stream, shared_secret).await?;
    let packet = from_packet_bytes(&packet_data)?;

    let Packets::AgentAuth(auth_packet) = packet else {
        let resp = AgentAuthResponse::new(false, "Expected AgentAuth packet".to_string());
        send_encrypted_packet_stream(stream, shared_secret, &resp.serialize()?).await?;
        return Err("Expected AgentAuth packet".into());
    };

    // Verify token
    match super::super::services::verify_agent_token(
        state,
        &auth_packet.audit_id,
        &auth_packet.token,
    ) {
        Ok(agent) => {
            let resp = AgentAuthResponse::new(true, "Authenticated".to_string());
            send_encrypted_packet_stream(stream, shared_secret, &resp.serialize()?).await?;
            Ok(agent)
        }
        Err(e) => {
            warn!("Agent auth failed for audit {}: {e}", auth_packet.audit_id);
            let resp = AgentAuthResponse::new(false, format!("{e}"));
            send_encrypted_packet_stream(stream, shared_secret, &resp.serialize()?).await?;
            Err(e.into())
        }
    }
}

/// Main multiplex loop: receives encrypted packets and dispatches them.
/// Runs until the connection closes or the agent is shut down (revoked).
#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
async fn multiplex_loop(
    mut reader: OwnedReadHalf,
    writer: &Arc<Mutex<OwnedWriteHalf>>,
    shared_secret: &[u8; 32],
    shutdown_rx: &mut tokio::sync::watch::Receiver<bool>,
    last_heartbeat: &Arc<std::sync::Mutex<Option<DateTime<Utc>>>>,
    latency_ms: &Arc<std::sync::Mutex<Option<u64>>>,
    shell_stream_id: &Arc<std::sync::Mutex<Option<u32>>>,
    system_info: &Arc<std::sync::Mutex<Option<SystemInfo>>>,
    io: Option<&SocketIo>,
    audit_id: &str,
    agent_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        tokio::select! {
            result = read_encrypted_packet(&mut reader, shared_secret) => {
                match result {
                    Ok(data) if data.is_empty() => {
                        info!("Agent sent empty packet, closing connection");
                        return Ok(());
                    }
                    Ok(data) => {
                        match from_packet_bytes(&data) {
                            Ok(Packets::Heartbeat(received_hb)) => {
                                if let Ok(mut hb) = last_heartbeat.lock() {
                                    *hb = Some(Utc::now());
                                }
                                if received_hb.latency_ms > 0 && let Ok(mut lat) = latency_ms.lock() {
                                    *lat = Some(received_hb.latency_ms);
                                }
                                // Echo back with the same sent_at_ms so the agent can measure RTT
                                let echo = shared::packets::Heartbeat::with_timing(received_hb.sent_at_ms, 0);
                                send_encrypted_packet(writer, shared_secret, &echo.serialize()?).await?;
                            }
                            Ok(Packets::SystemInfo(info)) => {
                                info!(
                                    "Received system info: hostname={}, os={} {}, cpus={}, ram={}MB",
                                    info.hostname,
                                    info.os_name,
                                    info.os_version,
                                    info.cpu_count,
                                    info.total_memory_bytes / 1_048_576,
                                );
                                if let Ok(mut si) = system_info.lock() {
                                    *si = Some(*info);
                                }
                            }
                            Ok(Packets::StreamData(stream_data)) => {
                                if let Some(io) = io {
                                    crate::api::ws::events::agent::emit_agent_output(
                                        io,
                                        audit_id,
                                        agent_id,
                                        stream_data.stream_id,
                                        &stream_data.data,
                                    )
                                    .await;
                                }
                            }
                            Ok(Packets::StreamClose(close)) => {
                                // Clear shell stream if it's the one being closed
                                if let Ok(mut sid) = shell_stream_id.lock() && *sid == Some(close.stream_id) {
                                    *sid = None;
                                }
                                if let Some(io) = io {
                                    crate::api::ws::events::agent::emit_agent_stream_closed(
                                        io,
                                        audit_id,
                                        agent_id,
                                        close.stream_id,
                                    )
                                    .await;
                                }
                            }
                            Ok(Packets::KeyloggerData(keylog)) => {
                                if let Some(io) = io {
                                    crate::api::ws::events::agent::emit_agent_keylogger(
                                        io,
                                        audit_id,
                                        agent_id,
                                        &keylog.data,
                                    )
                                    .await;
                                }
                            }
                            Ok(Packets::StreamOpen(open)) => {
                                tracing::debug!("Agent opened shell stream: {}", open.stream_id);
                                if let Ok(mut sid) = shell_stream_id.lock() {
                                    *sid = Some(open.stream_id);
                                }
                            }
                            Ok(packet) => {
                                tracing::debug!("Received packet: {packet:?}");
                            }
                            Err(e) => {
                                error!("Failed to parse packet: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Agent shutdown requested (revoked)");
                    // Send AgentControl packet to agent before disconnecting
                    let shutdown_packet = AgentControl::shutdown("Revoked by server".to_string());
                    if let Err(e) = send_encrypted_packet(writer, shared_secret, &shutdown_packet.serialize()?).await {
                        warn!("Failed to send shutdown packet to agent: {e}");
                    }
                    return Ok(());
                }
            }
        }
    }
}
