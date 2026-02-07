use std::vec;

use crate::api::swagger::ApiDoc;
use axum::{Extension, Router, routing::get};
use http::{HeaderName, HeaderValue, Method};
use socketioxide::SocketIo;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{audits, auth, events, metrics, status, teams, users, ws};
use crate::middlewares::{CsrfProtection, event_logging_middleware, metrics_middleware};
use crate::state::GlobalState;
use crate::utils::init::get_state;

#[allow(dead_code)]
pub async fn get_router() -> Router {
    let state = get_state().await;
    get_router_with_state(&state)
}

pub fn get_router_with_state(state: &GlobalState) -> Router {
    let secrets = state.get_secrets();

    // Initialize Prometheus metrics recorder
    metrics::install_recorder(state.clone());

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
            HeaderName::from_static("x-csrf-token"),
        ])
        .expose_headers(vec![HeaderName::from_static("x-csrf-token")])
        .allow_credentials(true);

    let csrf_protection =
        std::sync::Arc::new(CsrfProtection::new(state.csrf_secret.as_ref().clone()));

    let (ws_layer, io) = SocketIo::builder()
        .with_state(state.clone())
        .build_layer();
    state.set_io(io.clone());
    ws::setup_ws(&io);

    let router_v1 = Router::new()
        .nest("/audits", audits::get_routes(state))
        .nest("/teams", teams::get_routes(state))
        .nest("/users", users::get_routes(state))
        .nest("/events", events::get_routes(state))
        .nest("/auth", auth::get_routes(state.clone()))
        .nest("/status", status::get_routes(state.clone()))
        .route("/csrf", get(crate::api::csrf::get_csrf));

    let main_router = Router::new()
        .nest("/v1", router_v1)
        .layer(Extension(csrf_protection.clone()))
        .layer(CookieManagerLayer::new())
        .layer(axum::middleware::from_fn_with_state(
            csrf_protection,
            crate::middlewares::csrf_middleware,
        ))
        .fallback(|| async move {
            tracing::warn!("404 fallback hit for unmatched route");
            (axum::http::StatusCode::NOT_FOUND, "error.route.notFound")
        })
        .layer(ws_layer)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            event_logging_middleware,
        ))
        .layer(axum::middleware::from_fn(metrics_middleware))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
        .layer(CookieManagerLayer::new());

    let is_prod = std::env::var("VAULT_ENV").unwrap_or("local".to_string()) == "prod";

    let final_router = if is_prod {
        main_router
    } else {
        main_router
            .merge(SwaggerUi::new("/v1/swagger").url("/v1/swagger/openapi.json", ApiDoc::openapi()))
    };

    // Metrics available in all environments
    final_router.merge(metrics::get_routes())
}
