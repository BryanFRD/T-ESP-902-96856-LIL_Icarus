use socketioxide::extract::SocketRef;
use tracing::{info, warn};

use crate::api::auth::models::JwtClaims;

/// Error returned when WebSocket authentication fails.
#[derive(Debug)]
pub struct WsAuthError(pub String);

impl std::fmt::Display for WsAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for WsAuthError {}

/// Extract the value of a named cookie from the raw `Cookie` header.
fn extract_cookie_value<'a>(cookie_header: &'a str, name: &str) -> Option<&'a str> {
    cookie_header
        .split(';')
        .map(str::trim)
        .find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            if key.trim() == name {
                Some(value.trim())
            } else {
                None
            }
        })
}

/// Socket.IO connect middleware that authenticates users via the `auth` HTTP cookie.
///
/// On success the validated [`JwtClaims`] are stored in the socket extensions and
/// the socket automatically joins the `user:{user_id}` room.
pub async fn auth_middleware(socket: SocketRef) -> Result<(), WsAuthError> {
    let headers = &socket.req_parts().headers;

    // 1. Extract the Cookie header
    let cookie_header = headers
        .get(http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| WsAuthError("Missing Cookie header".to_string()))?;

    // 2. Extract the "auth" cookie value (the JWT)
    let jwt_token = extract_cookie_value(cookie_header, "auth")
        .ok_or_else(|| WsAuthError("Missing 'auth' cookie".to_string()))?;

    // 3. Decode & validate the JWT
    let jwt_secret =
        std::env::var("JWT_SECRET").map_err(|_| WsAuthError("JWT_SECRET not set".to_string()))?;

    let claims = JwtClaims::from_jwt(jwt_token, &jwt_secret);

    if !claims.is_valid() {
        warn!("WebSocket auth failed: invalid or expired JWT");
        return Err(WsAuthError("Invalid or expired JWT".to_string()));
    }

    let user_id = claims.sub.to_string();

    // 4. Store claims in socket extensions for later use by handlers
    socket.extensions.insert(claims);

    // 5. Auto-join the personal user room
    let user_room = format!("user:{user_id}");
    socket.join(user_room);

    info!(user_id = %user_id, socket_id = %socket.id, "WebSocket authenticated");

    Ok(())
}
