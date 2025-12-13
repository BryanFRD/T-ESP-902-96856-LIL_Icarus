#![allow(clippy::needless_for_each)]

mod api;
mod mails;
mod schema;
mod state;
mod utils;
mod vault;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    utils::start_logger();
    tracing::info!("Starting server...");

    let app = utils::get_router().await;

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
