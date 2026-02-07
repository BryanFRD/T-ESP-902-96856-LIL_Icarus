use axum::extract::State as AxumState;
use serde_json::Value;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State as SocketState},
};
use tracing::{error, info, warn};

use crate::api::auth::JwtClaims;
use crate::api::notifications::{MarkAsReadPayload, mark_as_read};
use crate::state::GlobalState;

#[derive(Clone, Debug)]
struct UserSocketState {
    user_id: String,
}

pub fn setup_ws(state: &GlobalState) {
    info!("Setting up Socket.IO namespace");
    
    let io = state.io.as_ref();
    let state_clone = state.clone();
    
    io.ns("/v1/ws", move |socket: SocketRef| {
        on_connect(socket, state_clone.clone())
    });
}

fn on_connect(socket: SocketRef, state: GlobalState) {
    // Extract JWT from cookies
    let auth_cookie = socket.req_parts().headers
        .get("cookie")
        .and_then(|c| c.to_str().ok())
        .and_then(extract_jwt_from_cookie);
    
    let user_id = match auth_cookie {
        Some(jwt) => {
            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();
            let claims = JwtClaims::from_jwt(&jwt, &jwt_secret);
            
            if !claims.is_valid() {
                warn!("Invalid JWT token for socket connection");
                socket.disconnect().ok();
                return;
            }
            
            claims.sub.to_string()
        }
        None => {
            warn!("No JWT token found in socket connection");
            socket.disconnect().ok();
            return;
        }
    };

    // Store user_id in socket extensions (State)
    socket.extensions.insert(UserSocketState { 
        user_id: user_id.clone() 
    });

    // Join user's personal room for targeted notifications
    if let Err(e) = socket.join(user_id.clone()) {
        error!("Failed to join room for user {}: {}", user_id, e);
    }

    info!("User {} connected via WebSocket (socket: {})", user_id, socket.id);

    // Handler for marking notifications as read
    let state_clone = state.clone();
    socket.on("notification:read", 
        move |socket: SocketRef, Data::<MarkAsReadPayload>(payload)| {
            let state = state_clone.clone();
            async move {
                let user_state = match socket.extensions.get::<UserSocketState>() {
                    Some(state) => state,
                    None => {
                        error!("No user state found in socket extensions");
                        return;
                    }
                };
                
                match mark_as_read(&state, &payload.notification_id, &user_state.user_id) {
                    Ok(()) => {
                        info!("Marked notification {} as read for user {}", 
                              payload.notification_id, user_state.user_id);
                        socket.emit("notification:read:success", &payload.notification_id).ok();
                    }
                    Err(e) => {
                        error!("Failed to mark notification as read: {:?}", e);
                        socket.emit("notification:read:error", "Failed to mark as read").ok();
                    }
                }
            }
        }
    );

    socket.on_disconnect(|socket: SocketRef| async move {
        if let Some(user_state) = socket.extensions.get::<UserSocketState>() {
            info!("User {} disconnected (socket: {})", user_state.user_id, socket.id);
        }
    });
}

/// Extract JWT token from cookie header string
fn extract_jwt_from_cookie(cookie_header: &str) -> Option<String> {
    cookie_header
        .split(';')
        .map(str::trim)
        .find(|c| c.starts_with("jwt="))
        .and_then(|c| c.strip_prefix("jwt="))
        .map(String::from)
}
