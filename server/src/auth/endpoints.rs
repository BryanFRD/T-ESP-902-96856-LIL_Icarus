use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
#[cfg(not(debug_assertions))]
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, cookie::time};
use tracing::error;
use validator::Validate;

use crate::state::GlobalState;

use super::{
    models::{JwtClaims, LoginPayload, RegisterPayload},
    services::{create_user, get_user_by_email, user_exists},
};

pub async fn login(
    State(state): State<GlobalState>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    if let Err(_) = payload.validate() {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let user = match get_user_by_email(&state, &payload.email) {
        Ok(user) => match user {
            Some(user) => user,
            None => return (StatusCode::UNAUTHORIZED).into_response(),
        },
        Err(e) => {
            error!("Failed to get user by email: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if !user.verify_password(&payload.password) {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let jwt = JwtClaims::from(user);
    let jwt = match jwt.to_string(state.get_secrets().jwt_secret.as_str()) {
        Ok(jwt) => jwt,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    let mut cookie = Cookie::new("auth", jwt);

    #[cfg(not(debug_assertions))]
    cookie.set_secure(true);
    #[cfg(not(debug_assertions))]
    cookie.set_same_site(SameSite::None);
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    (StatusCode::OK).into_response()
}

pub async fn register(
    State(state): State<GlobalState>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    if let Err(_) = payload.validate() {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    match user_exists(&state, &payload) {
        Ok(None) => (),
        Ok(Some(val)) => return (StatusCode::CONFLICT, val).into_response(),
        Err(e) => {
            error!("Failed to check if user exists: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    match create_user(&state, payload.into()) {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => {
            error!("Failed to create user: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }
}

pub async fn me(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    let user = match get_user_by_email(&state, &jwt.email) {
        Ok(user) => match user {
            Some(user) => user,
            None => return (StatusCode::UNAUTHORIZED).into_response(),
        },
        Err(e) => {
            error!("Failed to get user by email: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    Json(user).into_response()
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::new("auth", "");
    cookie.set_expires(time::OffsetDateTime::UNIX_EPOCH);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookies.add(cookie);

    (StatusCode::OK).into_response()
}
