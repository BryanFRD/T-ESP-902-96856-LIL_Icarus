use std::vec;

use axum::Router;
use axum::middleware::from_fn;
use http::{HeaderName, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::state::GlobalState;
use crate::vault::get_secrets;
use crate::{auth, status};

pub fn start_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .compact()
        .init();
}

pub async fn get_state() -> GlobalState {
    let secrets = match get_secrets().await {
        Ok(secrets) => secrets,
        Err(e) => {
            tracing::error!("Failed to get secrets: {}", e);
            panic!("Failed to get secrets");
        }
    };
    GlobalState::new(secrets)
}

pub async fn get_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);

    let state = get_state().await;

    let router_v1 = Router::new()
        .nest("/auth", auth::get_routes(state.clone()))
        .layer(from_fn(auth::middleware))
        .nest("/status", status::get_routes(state.clone()))
        .nest("/auth", auth::get_no_auth_routes(state.clone()));

    Router::new()
        .nest("/v1", router_v1)
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
}
