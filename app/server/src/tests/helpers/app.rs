#[cfg(test)]
use crate::api::status::StatusEnum;
#[cfg(test)]
use crate::state::GlobalState;
#[cfg(test)]
use crate::vault::Secrets;
#[cfg(test)]
use crate::{
    api::{audits, auth, events, status, teams, users},
    tests::helpers::db::DbPool,
};
#[cfg(test)]
use axum::{Router, extract::connect_info::MockConnectInfo};
#[cfg(test)]
use http::{HeaderName, HeaderValue, Method};
#[cfg(test)]
use std::net::SocketAddr;
#[cfg(test)]
use tower_cookies::CookieManagerLayer;
#[cfg(test)]
use tower_http::cors::CorsLayer;

/// Crée des secrets de test avec des valeurs par défaut
#[cfg(test)]
pub fn create_test_secrets(database_url: String) -> Secrets {
    Secrets {
        database_url,
        jwt_secret: "test_jwt_secret_key_for_testing_only_12345".to_string(),
        status: StatusEnum::Up,
        status_message: "Test server is up".to_string(),
        origin_cors: "http://localhost:3000,http://localhost:8080".to_string(),
        resend_api_key: "test_resend_api_key".to_string(),
        event_encryption_key: "6QpTjImC9TYLc08k+pw+PqZ/ysdRQkZYXD7XxQ3Zmeo=".to_string(),
    }
}

/// Crée un `GlobalState` pour les tests avec le pool de DB fourni et des mocks
#[cfg(test)]
pub fn create_test_state(db_pool: DbPool, database_url: String) -> GlobalState {
    let secrets = create_test_secrets(database_url);
    GlobalState::new_with_pool(db_pool, secrets)
}

/// Crée un `GlobalState` pour les tests avec uniquement le pool (déduit l'URL de la DB)
#[cfg(test)]
pub fn create_test_state_from_pool(db_pool: DbPool) -> GlobalState {
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("LOCAL_DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/postgres".to_string());
    create_test_state(db_pool, database_url)
}

/// Crée le router Axum complet pour les tests d'intégration
#[cfg(test)]
pub fn create_test_router(state: GlobalState) -> Router {
    let secrets = state.get_secrets();

    let allowed_origins: Vec<HeaderValue> = secrets
        .origin_cors
        .split(',')
        .map(str::trim)
        .map(|s| HeaderValue::from_str(s).expect("Invalid CORS origin"))
        .collect();

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
        .nest("/audits", audits::get_routes(&state))
        .nest("/teams", teams::get_routes(&state))
        .nest("/users", users::get_routes(&state))
        .nest("/auth", auth::get_routes(state.clone()))
        .nest("/events", events::get_routes(&state))
        .nest("/status", status::get_routes(state.clone()));

    let test_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    Router::new()
        .nest("/v1", router_v1)
        .layer(cors)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::middlewares::event_logging_middleware,
        ))
        .layer(CookieManagerLayer::new())
        .layer(MockConnectInfo(test_addr))
}
