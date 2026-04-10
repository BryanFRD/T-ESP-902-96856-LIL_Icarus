#![allow(clippy::needless_for_each)]

mod api;
mod helpers;
mod mails;
mod middlewares;
mod migrations;
mod schema;
mod state;
mod utils;
mod vault;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    utils::start_logger();
    tracing::info!("Starting server test...");

    let state = utils::init::get_state().await;
    let app = utils::router::get_router_with_state(&state);

    // Spawn the agent TCP listener on port 1337
    let tcp_state = state.clone();
    tokio::spawn(async move {
        api::agents::tcp::listener::start_tcp_listener(tcp_state).await;
    });

    tokio::spawn(async move {
        utils::start_cleanup_task(state).await;
    });

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => {
            tracing::info!("Listening on {}", listener.local_addr().unwrap());
            listener
        }
        Err(e) => {
            tracing::error!("Failed to bind to address: {}", e);
            return;
        }
    };

    match axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Server error: {e}");
        }
    }
}
