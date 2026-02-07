use socketioxide::SocketIo;
use socketioxide::extract::SocketRef;
use socketioxide::handler::ConnectHandler;
use tracing::info;

use crate::api::auth::models::JwtClaims;

use super::auth::auth_middleware;
use super::rooms::handlers;

/// Register the Socket.IO namespace, authentication middleware, and event handlers.
pub fn setup_ws(io: &SocketIo) {
    info!("Setting up Socket.IO namespace /v1/ws");
    io.ns("/v1/ws", on_connect.with(auth_middleware));
}

/// Called once a socket has passed the authentication middleware.
async fn on_connect(socket: SocketRef) {
    let user_id = socket
        .extensions
        .get::<JwtClaims>()
        .map_or_else(|| "unknown".to_string(), |c| c.sub.to_string());

    info!(
        user_id = %user_id,
        socket_id = %socket.id,
        "Socket connected"
    );

    socket.on("room:join", handlers::on_room_join);
    socket.on("room:leave", handlers::on_room_leave);

    socket.on_disconnect(on_disconnect);
}

/// Called when a socket disconnects. Rooms are cleaned up automatically by socketioxide.
async fn on_disconnect(socket: SocketRef) {
    let user_id = socket
        .extensions
        .get::<JwtClaims>()
        .map_or_else(|| "unknown".to_string(), |c| c.sub.to_string());

    info!(
        user_id = %user_id,
        socket_id = %socket.id,
        "Socket disconnected"
    );
}
