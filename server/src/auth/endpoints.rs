use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
#[cfg(not(debug_assertions))]
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, cookie::time};
use tracing::error;
use validator::Validate;

use crate::state::GlobalState;

use super::{
    models::{JwtClaims, LoginPayload, RegisterPayload, UpdatePayload, User},
    services::{
        check_update_conflicts, create_user, delete_user, get_user_by_email, get_user_by_id,
        hash_password, update_user_fields, user_exists, validate_update_payload,
    },
};

#[utoipa::path(
    post,
    path = "/v1/auth/login",
    description = "Login a user",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Logged in successfully"),
        (status = BAD_REQUEST, description = "Invalid request payload"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
    ),
    tag = "Auth"
)]
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
    cookie.set_expires(time::OffsetDateTime::now_utc() + time::Duration::days(7));
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    (StatusCode::OK).into_response()
}

#[utoipa::path(
    post,
    path = "/v1/auth/register",
    description = "Register a new user",
    request_body = RegisterPayload,
    responses(
        (status = 201, description = "Registered successfully"),
        (status = BAD_REQUEST, description = "Invalid request payload"),
        (status = CONFLICT, description = "User already exists"),
    ),
    tag = "Auth"
)]
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

#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "Auth",
)]
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::new("auth", "");

    #[cfg(not(debug_assertions))]
    cookie.set_secure(true);
    #[cfg(not(debug_assertions))]
    cookie.set_same_site(SameSite::None);
    cookie.set_expires(time::OffsetDateTime::UNIX_EPOCH);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookies.add(cookie);

    (StatusCode::OK).into_response()
}

#[utoipa::path(
    get,
    path = "/v1/auth/me",
    responses(
        (status = 200, description = "User information retrieved successfully", body = User),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
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

#[utoipa::path(
    patch,
    path = "/v1/auth/me",
    description = "Update user information",
    request_body = UpdatePayload,
    responses(
        (status = 200, description = "User information updated successfully"),
        (status = BAD_REQUEST, description = "Invalid request payload"),
        (status = CONFLICT, description = "Update conflicts with existing data"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
pub async fn update_me(
    State(state): State<GlobalState>,
    jwt: JwtClaims,
    Json(payload): Json<UpdatePayload>,
) -> impl IntoResponse {
    if let Err(_) = validate_update_payload(&payload) {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let user = match get_user_by_id(&state, &jwt.sub.to_string()) {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED).into_response(),
        Err(e) => {
            error!("Failed to get user by email: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if payload.new_password.is_some() {
        if let Some(ref current_pw) = payload.password {
            let hashed = hash_password(current_pw);
            if hashed != user.password_hash {
                return (StatusCode::UNAUTHORIZED, "Current password is incorrect").into_response();
            }
        }
    }

    match check_update_conflicts(&state, &payload, &user) {
        Ok(Some(conflict)) => return (StatusCode::CONFLICT, conflict).into_response(),
        Ok(None) => (),
        Err(e) => {
            error!("Failed to check update conflicts: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match update_user_fields(&state, &user.id, &payload) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to update user: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if let Err(_) = update_user_fields(&state, &user.id, &payload) {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    (StatusCode::OK).into_response()
}

pub async fn delete_me(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    let user_id = jwt.sub.to_string();

    match delete_user(&state, &user_id) {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Failed to delete user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
