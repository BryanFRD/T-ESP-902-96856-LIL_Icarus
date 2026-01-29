use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tracing::{info, error};

pub fn setup_ws(io: &SocketIo) {
    info!("Setting up Socket.IO namespace");
    io.ns("/v1/ws", on_connect);
}

async fn on_connect(socket: SocketRef) {
    socket.on("message", |s: SocketRef, Data::<Value>(data)| async move {
        if let Err(e) = s.emit("message", &data) {
            error!("Failed to emit message to {}: {}", s.id, e);
        }
    });

    socket.on_disconnect(|_: SocketRef| async move {
    });
}
