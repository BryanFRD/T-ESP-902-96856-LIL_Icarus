use std::vec;

use crate::api::swagger::ApiDoc;
use axum::Router;
use http::{HeaderName, HeaderValue, Method};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{audits, auth, events, status, teams, users};
use crate::middlewares::{CsrfProtection, event_logging_middleware};
use crate::state::GlobalState;
use crate::utils::init::get_state;

#[allow(dead_code)]
pub async fn get_router() -> Router {
    let state = get_state().await;
    get_router_with_state(&state)
}

pub fn get_router_with_state(state: &GlobalState) -> Router {
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
            HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true);

    let csrf_protection =
        std::sync::Arc::new(CsrfProtection::new(state.csrf_secret.as_ref().clone()));

    let router_v1 = Router::new()
        .nest("/audits", audits::get_routes(state))
        .nest("/teams", teams::get_routes(state))
        .nest("/users", users::get_routes(state))
        .nest("/events", events::get_routes(state))
        .nest("/auth", auth::get_routes(state.clone()))
        .nest("/status", status::get_routes(state.clone()));

    let main_router = Router::new()
        .nest("/v1", router_v1)
        .layer(axum::middleware::from_fn_with_state(
            csrf_protection,
            crate::middlewares::csrf_middleware,
        ))
        .layer(CookieManagerLayer::new())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            event_logging_middleware,
        ))
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors);

    let is_prod = std::env::var("VAULT_ENV").unwrap_or("local".to_string()) == "prod";

    if is_prod {
        main_router
    } else {
        main_router
            .merge(SwaggerUi::new("/v1/swagger").url("/v1/swagger/openapi.json", ApiDoc::openapi()))
    }
}
