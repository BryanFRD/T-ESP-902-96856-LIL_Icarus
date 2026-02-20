use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use tower_cookies::Cookies;

use super::{get_user_by_id_or_error, get_user_or_error};
use crate::helpers::{
    bad_request_error, conflict_error, create_auth_cookie, internal_error, not_found_error,
    unauthorized_error,
};

use crate::api::{
    auth::JwtClaims,
    users::{UserError, UserResponse, avatar_url_path, get_all_users, models::AvatarForm},
};
use crate::state::GlobalState;

use super::{
    check_update_conflicts, delete_user, get_user_by_id, models::UpdatePayload, update_user_fields,
    validate_update_payload,
};

#[utoipa::path(
    get,
    path = "/v1/users/me",
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserResponse),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
pub async fn me(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    let user = match get_user_or_error(&state, &jwt.email) {
        Ok(user) => user,
        Err(e) => return e,
    };

    Json(UserResponse::from(user)).into_response()
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
    match validate_update_payload(&payload) {
        Ok(()) => (),
        Err(UserError::PayloadValidationError(e)) => {
            return bad_request_error(&format!("error.user.update.validate.{e}"));
        }
        Err(e) => {
            return internal_error("Failed to validate payload", e);
        }
    }

    let user = match get_user_by_id_or_error(&state, &jwt.sub.to_string()) {
        Ok(user) => user,
        Err(e) => return e,
    };

    if payload.new_password.is_some()
        && let Some(ref current_pw) = payload.password
    {
        // Verify old password using migration-aware verification
        if !user.verify_password(current_pw) {
            return unauthorized_error();
        }
    }

    match check_update_conflicts(&state, &payload, &user) {
        Ok(Some(conflict)) => {
            return conflict_error(&format!("error.user.update.conflict.{conflict}"));
        }
        Ok(None) => (),
        Err(e) => {
            return internal_error("Failed to check update conflicts", e);
        }
    }

    if let Err(e) = update_user_fields(&state, &user.id, &payload) {
        return internal_error("Failed to update user", e);
    }

    if payload.email.is_none() {
        return (StatusCode::OK).into_response();
    }

    // If email was updated, we need to re-issue the JWT with the new email
    let updated_user = match get_user_by_id_or_error(&state, &user.id) {
        Ok(user) => user,
        Err(e) => return e,
    };

    let jwt = JwtClaims::from(updated_user);
    let Ok(jwt) = jwt.to_string(state.get_secrets().jwt_secret.as_str()) else {
        return internal_error("Failed to create JWT", "JWT creation failed");
    };

    create_auth_cookie(jwt, &cookies);
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
        Err(e) => internal_error("Failed to delete user", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/users",
    responses(
        (status = 200, description = "List of users retrieved successfully", body = [UserResponse]),
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
        Ok(users) => {
            let users = users
                .into_iter()
                .map(UserResponse::from)
                .collect::<Vec<_>>();
            Json(users).into_response()
        }
        Err(e) => internal_error("Failed to get users", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/users/{id}",
    responses(
        (status = 200, description = "User information retrieved successfully", body = UserResponse),
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
        Ok(Some(user)) => Json(UserResponse::from(user)).into_response(),
        Ok(None) => not_found_error("error.user.admin.notFound"),
        Err(e) => internal_error("Failed to get user by ID", e),
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
    match validate_update_payload(&payload) {
        Ok(()) => (),
        Err(UserError::PayloadValidationError(e)) => {
            return bad_request_error(&format!("error.user.update.validate.{e}"));
        }
        Err(e) => {
            return internal_error("Failed to validate payload", e);
        }
    }

    let user = match get_user_by_id(&state, &id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return not_found_error("error.user.admin.notFound");
        }
        Err(e) => {
            return internal_error("Failed to get user by ID", e);
        }
    };

    match check_update_conflicts(&state, &payload, &user) {
        Ok(Some(conflict)) => {
            return conflict_error(&format!("error.user.update.conflict.{conflict}"));
        }
        Ok(None) => (),
        Err(e) => {
            return internal_error("Failed to check update conflicts", e);
        }
    }

    match update_user_fields(&state, &user.id, &payload) {
        Ok(()) => Json(UserResponse::from(user)).into_response(),
        Err(e) => internal_error("Failed to update user", e),
    }
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
        Err(e) => internal_error("Failed to delete user", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/users/me/avatar",
    description = "Upload user avatar (automatically converted to WebP format with 80% quality)",
    request_body(content = AvatarForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Avatar uploaded and converted successfully"),
        (status = BAD_REQUEST, description = "Invalid file, file too large, or processing failed"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to save avatar"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
#[axum::debug_handler]
pub async fn upload_avatar(
    State(state): State<GlobalState>,
    jwt: JwtClaims,
    multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let user_id = jwt.sub.to_string();

    let current_user = match get_user_by_id_or_error(&state, &user_id) {
        Ok(user) => user,
        Err(e) => return e,
    };

    if let Some(ref old_avatar) = current_user.avatar_url {
        super::delete_avatar_file(old_avatar);
    }

    match super::save_avatar(&state, &user_id, multipart).await {
        Ok(filename) => Json(serde_json::json!({
            "avatar_url": avatar_url_path(&filename)
        }))
        .into_response(),
        Err(UserError::MultipartError(e)) => {
            // error!("Failed to save avatar: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(format!("error.user.update.avatar.{e}")),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/users/me/avatar",
    description = "Delete user avatar",
    responses(
        (status = 204, description = "Avatar deleted successfully"),
        (status = UNAUTHORIZED, description = "Invalid credentials"),
        (status = NOT_FOUND, description = "No avatar to delete"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to delete avatar"),
    ),
    security(
        ("cookieAuth" = [])
    ),
    tag = "User",
)]
pub async fn delete_avatar(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    let user_id = jwt.sub.to_string();

    let current_user = match get_user_by_id_or_error(&state, &user_id) {
        Ok(user) => user,
        Err(e) => return e,
    };

    let Some(avatar_filename) = current_user.avatar_url else {
        return (
            StatusCode::NOT_FOUND,
            Json("error.user.avatar.noAvatarToDelete"),
        )
            .into_response();
    };

    super::delete_avatar_file(&avatar_filename);

    match super::update_user_avatar(&state, &user_id, "") {
        Ok(()) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => internal_error("Failed to update user avatar in database", e),
    }
}
