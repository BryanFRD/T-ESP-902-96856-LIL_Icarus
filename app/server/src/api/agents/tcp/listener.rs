use crate::state::GlobalState;
use tokio::net::TcpListener;
use tracing::{error, info};

use super::handler;

/// Start the TCP listener for agent connections on port 1337.
///
/// This runs in a loop accepting connections and spawning a handler task for each.
pub async fn start_tcp_listener(state: GlobalState) {
    let listener = match TcpListener::bind("0.0.0.0:1337").await {
        Ok(l) => {
            info!("Agent TCP listener started on {}", l.local_addr().unwrap());
            l
        }
        Err(e) => {
            error!("Failed to bind agent TCP listener on :1337: {e}");
            return;
        }
    };

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let state = state.clone();
                tokio::spawn(async move {
                    if let Err(e) = handler::handle_agent_connection(stream, addr, state).await {
                        error!("Agent connection error from {addr}: {e}");
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept agent connection: {e}");
            }
        }
    }
}
