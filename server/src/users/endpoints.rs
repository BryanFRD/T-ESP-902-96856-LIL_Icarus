use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
#[cfg(not(debug_assertions))]
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, cookie::time};
use tracing::error;

use crate::{
    auth::{JwtClaims, hash_password},
    state::GlobalState,
    users::get_all_users,
};

use super::{
    User, check_update_conflicts, delete_user, get_user_by_email, get_user_by_id,
    models::UpdatePayload, update_user_fields, validate_update_payload,
};

#[utoipa::path(
    get,
    path = "/v1/users/me",
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
    path = "/v1/users/me",
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
    cookies: Cookies,
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

    if payload.email.is_none() {
        return (StatusCode::OK).into_response();
    }

    // If email was updated, we need to re-issue the JWT with the new email
    let updated_user = match get_user_by_id(&state, &user.id) {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        Err(e) => {
            error!("Failed to get updated user: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let jwt = JwtClaims::from(updated_user);
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
    delete,
    path = "/v1/users/me",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to delete user"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
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

#[utoipa::path(
    get,
    path = "/v1/users",
    responses(
        (status = 200, description = "List of users retrieved successfully", body = [User]),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = FORBIDDEN, description = "Access denied")
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "Admin",
)]
pub async fn list_users_admin(State(state): State<GlobalState>) -> impl IntoResponse {
    match get_all_users(&state) {
        Ok(users) => Json(users).into_response(),
        Err(e) => {
            error!("Failed to get users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    responses(
        (status = 200, description = "User information retrieved successfully", body = User),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = FORBIDDEN, description = "Access denied"),
        (status = NOT_FOUND, description = "User not found")
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "Admin",
)]
pub async fn get_user_admin(
    State(state): State<GlobalState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match get_user_by_id(&state, &id) {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(e) => {
            error!("Failed to get user by ID: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[utoipa::path(
    patch,
    path = "/v1/users/{id}",
    description = "Update user information by admin",
    request_body = UpdatePayload,
    responses(
        (status = 200, description = "User information updated successfully"),
        (status = BAD_REQUEST, description = "Invalid request payload"),
        (status = CONFLICT, description = "Update conflicts with existing data"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = FORBIDDEN, description = "Access denied"),
        (status = NOT_FOUND, description = "User not found")
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "Admin",
)]
pub async fn update_user_admin(
    State(state): State<GlobalState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePayload>,
) -> impl IntoResponse {
    if let Err(_) = validate_update_payload(&payload) {
        return (StatusCode::BAD_REQUEST).into_response();
    }

    let user = match get_user_by_id(&state, &id) {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::NOT_FOUND).into_response(),
        Err(e) => {
            error!("Failed to get user by ID: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if payload.new_password.is_some() && payload.password.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            "Current password is required to set a new password",
        )
            .into_response();
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

    Json(user).into_response()
}

#[utoipa::path(
    delete,
    path = "/v1/users/{id}",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = FORBIDDEN, description = "Access denied"),
        (status = NOT_FOUND, description = "User not found"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to delete user")
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "Admin",
)]
pub async fn delete_user_admin(
    State(state): State<GlobalState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match delete_user(&state, &id) {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Failed to delete user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
