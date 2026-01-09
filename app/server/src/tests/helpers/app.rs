#[cfg(test)]
use crate::api::status::StatusEnum;
#[cfg(test)]
use crate::state::GlobalState;
#[cfg(test)]
use crate::vault::Secrets;
#[cfg(test)]
use crate::{api::{auth, status, teams, users}, tests::helpers::db::DbPool};
#[cfg(test)]
use axum::Router;
#[cfg(test)]
use http::{HeaderName, HeaderValue, Method};
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
    }
}

/// Crée un `GlobalState` pour les tests avec le pool de DB fourni et des mocks
#[cfg(test)]
pub fn create_test_state(db_pool: DbPool, database_url: String) -> GlobalState {
    let secrets = create_test_secrets(database_url);
    GlobalState::new_with_pool(db_pool, secrets)
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
        .nest("/teams", teams::get_routes(&state))
        .nest("/users", users::get_routes(&state))
        .nest("/auth", auth::get_routes(state.clone()))
        .nest("/status", status::get_routes(state.clone()));

    Router::new()
        .nest("/v1", router_v1)
        .layer(CookieManagerLayer::new())
        .layer(cors)
}
