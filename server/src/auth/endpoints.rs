use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
use rand::Rng;
use std::fmt::Write;
#[cfg(not(debug_assertions))]
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, cookie::time};
use tracing::error;
use validator::Validate;

use crate::{
    auth::{
        models::ResetPasswordRequestPayload,
        services::{consume_token, get_token, save_reset_password_token},
    },
    mails::{send_confirmation_mail, send_reset_password_mail},
    state::GlobalState,
    users::{create_user, get_user_by_email, get_user_by_id, update_user_password, user_exists},
};

use super::models::{ConfirmEmailPayload, EmailConfirmationToken, JwtClaims, LoginPayload, RegisterPayload, ResetPasswordPayload};

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
    if payload.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json("error.user.login.invalidEmail"),
        )
            .into_response();
    }

    let user = match get_user_by_email(&state, &payload.email) {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json("error.user.auth.invalidCredentials"),
                )
                    .into_response();
            }
        },
        Err(e) => {
            error!("Failed to get user by email: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    };

    if !user.verify_password(&payload.password) {
        return (
            StatusCode::UNAUTHORIZED,
            Json("error.user.auth.invalidCredentials"),
        )
            .into_response();
    }

    let jwt = JwtClaims::from(user);
    let Ok(jwt) = jwt.to_string(state.get_secrets().jwt_secret.as_str()) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response();
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
    if payload.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json("error.user.register.invalidEmail"),
        )
            .into_response();
    }

    match user_exists(&state, &payload) {
        Ok(None) => (),
        Ok(Some(val)) => {
            return (
                StatusCode::CONFLICT,
                Json(format!("error.user.register.conflict.{val}")),
            )
                .into_response();
        }
        Err(e) => {
            error!("Failed to check if user exists: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    }

    let user: crate::users::User = payload.into();

    let token = EmailConfirmationToken::new(
        user.username.clone(),
        user.first_name.clone(),
        user.last_name.clone(),
        user.email.clone(),
        user.password_hash.clone(),
    );

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();
    let encrypted_token = match token.encrypt(jwt_secret.as_bytes()) {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to encrypt confirmation token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    };

    let resend = state.get_resend().lock().await;
    if let Err(e) = send_confirmation_mail(
        encrypted_token,
        user.first_name.clone(),
        user.email.clone(),
        &resend,
    )
    .await
    {
        error!("Failed to send confirmation email: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response();
    }

    (StatusCode::OK).into_response()
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
    post,
    path = "/v1/auth/request-password-reset",
    responses(
        (status = 200, description = "Reset password demand sent"),
        (status = 500, description = "Failed to send password reset demand")
    ),
    tag = "Auth",
)]
pub async fn request_password_reset(
    State(state): State<GlobalState>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> impl IntoResponse {
    let user = match get_user_by_email(&state, &payload.reset_email) {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::OK).into_response(),
        Err(e) => {
            error!("Error while getting user by email: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    };

    let token_bytes: [u8; 64];
    {
        let mut rng = state.get_rand().lock().await;
        token_bytes = rng.random();
    }

    // Crée un string, écrit les octets en format hexa et les ajoute à la string
    let token = token_bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    if save_reset_password_token(&state, &user.id, &token)
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response();
    }

    let resend = state.get_resend().lock().await;
    match send_reset_password_mail(token, user.first_name, payload.reset_email, &resend).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => {
            error!("Error while sending password mail: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/reset-password",
    responses(
        (status = 200, description = "Reset password demand sent"),
        (status = 500, description = "Failed to send password reset demand")
    ),
    tag = "Auth",
)]
pub async fn reset_password(
    State(state): State<GlobalState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> impl IntoResponse {
    let token = match get_token(&state, &payload.token).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("error.auth.resetPassword.invalidToken"),
            )
                .into_response();
        }
        Err(e) => {
            error!("Error while getting token: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    };

    if token.is_expired() {
        return (
            StatusCode::BAD_REQUEST,
            Json("error.auth.resetPassword.expiredToken"),
        )
            .into_response();
    }

    let user = match get_user_by_id(&state, &token.user_id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("error.auth.resetPassword.invalidToken"),
            )
                .into_response();
        }
        Err(e) => {
            error!("Error while getting user by id: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    };

    if consume_token(&state, &token.user_id).await.is_err() {
        error!("Error while consuming token");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response();
    }

    match update_user_password(&state, &user.id, &payload.new_password).await {
        Ok(()) => (StatusCode::OK).into_response(),
        Err(e) => {
            error!("Error while updating password: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/auth/confirm-email",
    request_body = ConfirmEmailPayload,
    responses(
        (status = 201, description = "Email confirmed and user created"),
        (status = BAD_REQUEST, description = "Invalid or expired token"),
        (status = CONFLICT, description = "User already exists"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    tag = "Auth",
)]
pub async fn confirm_email(
    State(state): State<GlobalState>,
    Json(payload): Json<ConfirmEmailPayload>,
) -> impl IntoResponse {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();

    let token = match EmailConfirmationToken::decrypt(&payload.token, jwt_secret.as_bytes()) {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to decrypt confirmation token: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json("error.auth.confirmEmail.invalidToken"),
            )
                .into_response();
        }
    };

    let check_payload = RegisterPayload {
        username: token.username.clone(),
        first_name: token.first_name.clone(),
        last_name: token.last_name.clone(),
        email: token.email.clone(),
        password: String::new(),
    };

    match user_exists(&state, &check_payload) {
        Ok(None) => (),
        Ok(Some(val)) => {
            return (
                StatusCode::CONFLICT,
                Json(format!("error.user.register.conflict.{val}")),
            )
                .into_response();
        }
        Err(e) => {
            error!("Failed to check if user exists: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response();
        }
    }

    let user = crate::users::User {
        id: ulid::Ulid::new().to_string(),
        username: token.username,
        first_name: token.first_name,
        last_name: token.last_name,
        email: token.email,
        role: crate::users::UserRole::Pentester,
        password_hash: token.password_hash,
        is_connected: false,
        avatar_url: None,
        created_at: chrono::Local::now().naive_utc(),
        updated_at: chrono::Local::now().naive_utc(),
        last_connected_at: chrono::Local::now().naive_utc(),
    };

    match create_user(&state, &user) {
        Ok(()) => (StatusCode::CREATED, Json("success.auth.confirmEmail")).into_response(),
        Err(e) => {
            error!("Failed to create user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}
