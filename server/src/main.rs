use axum::{Router, routing::get};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

mod utils;

#[tokio::main]
async fn main() {
    utils::start_logger();
    tracing::info!("Starting server...");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
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
