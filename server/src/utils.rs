use std::vec;

use crate::swagger::ApiDoc;
use axum::Router;
use axum::middleware::from_fn;
use http::{HeaderName, HeaderValue, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::GlobalState;
use crate::vault::get_secrets;
use crate::{auth, status, users};

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
    let state = get_state().await;
    let secrets = state.get_secrets();

    let allowed_origins: Vec<HeaderValue> = secrets
        .origin_cors
        .split(',')
        .map(str::trim)
        .map(HeaderValue::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| {
            tracing::error!("Invalid CORS origin: {}", e);
            panic!("Invalid CORS origin");
        });

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true);

    let router_v1 = Router::new()
        .nest("/users", users::get_routes(state.clone()))
        .nest("/auth", auth::get_routes(state.clone()))
        .layer(from_fn(auth::middleware))
        .nest("/status", status::get_routes(state.clone()))
        .nest("/auth", auth::get_no_auth_routes(state.clone()));

    let main_router = Router::new()
        .nest("/v1", router_v1)
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors);

    let is_prod = std::env::var("VAULT_ENV").unwrap_or("local".to_string()) == "prod";

    if !is_prod {
        main_router
            .merge(SwaggerUi::new("/v1/swagger").url("/v1/swagger/openapi.json", ApiDoc::openapi()))
    } else {
        main_router
    }
}
