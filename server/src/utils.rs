use axum::Router;
use axum::middleware::from_fn;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::auth;
use crate::state::GlobalState;

pub fn start_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .compact()
        .init();
}

pub fn get_state() -> GlobalState {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    GlobalState::new(&database_url)
}

pub fn get_router() -> Router {
    let state = get_state();

    let router_v1 = Router::new()
        .nest("/auth", auth::get_routes(state.clone()))
        .layer(from_fn(auth::middleware))
        .nest("/auth", auth::get_no_auth_routes(state.clone()));

    Router::new()
        .nest("/v1", router_v1)
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
