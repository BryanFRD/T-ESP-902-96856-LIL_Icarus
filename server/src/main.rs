mod auth;
mod schema;
mod state;
mod status;
mod swagger;
mod teams;
mod users;
mod utils;
mod vault;

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
        Ok(_) => {}
        Err(e) => {
            eprintln!("Server error: {}", e);
        }
    }
}
