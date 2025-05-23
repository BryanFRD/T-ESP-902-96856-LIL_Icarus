mod auth;
mod schema;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    utils::start_logger();
    tracing::info!("Starting server...");

    let app = utils::get_router();

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:80").await {
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
        Ok(_) => {}
        Err(e) => {
            eprintln!("Server error: {}", e);
        }
    }
}
