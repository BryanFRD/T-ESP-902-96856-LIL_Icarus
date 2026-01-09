#![allow(clippy::result_large_err)]
#![allow(dead_code)]

use axum::{response::{IntoResponse, Response}, Json};
use http::StatusCode;
#[cfg(not(debug_assertions))]
use tower_cookies::cookie::SameSite;
use tower_cookies::{cookie::time, Cookie, Cookies};
use tracing::error;

/// Creates an authentication cookie with proper security settings
///
/// # Arguments
/// * `jwt` - The JWT token string to store in the cookie
/// * `cookies` - The cookies jar to add the cookie to
///
/// # Security
/// In production (non-debug builds), sets Secure and SameSite=None flags
pub fn create_auth_cookie(jwt: String, cookies: &Cookies) {
    let mut cookie = Cookie::new("auth", jwt);

    #[cfg(not(debug_assertions))]
    cookie.set_secure(true);
    #[cfg(not(debug_assertions))]
    cookie.set_same_site(SameSite::None);
    cookie.set_expires(time::OffsetDateTime::now_utc() + time::Duration::days(7));
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);
}

/// Helper to handle database query results that return Option<T>
///
/// # Arguments
/// * `result` - The database query result
/// * `not_found_msg` - Error message key to return if the resource is not found
/// * `context` - Context string for error logging
///
/// # Returns
/// * `Ok(T)` if resource found
/// * `Err(Response)` with appropriate status code if not found or error occurred
pub fn handle_db_option<T>(
    result: Result<Option<T>, diesel::result::Error>,
    not_found_msg: &str,
    context: &str,
) -> Result<T, Response> {
    match result {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(not_found_msg)).into_response()),
        Err(e) => {
            error!("{}: {}", context, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response())
        }
    }
}

/// Helper to handle database query results that return Result<T, E>
///
/// # Arguments
/// * `result` - The database query result
/// * `context` - Context string for error logging
///
/// # Returns
/// * `Ok(T)` if successful
/// * `Err(Response)` with 500 status code if error occurred
pub fn handle_db_result<T>(
    result: Result<T, diesel::result::Error>,
    context: &str,
) -> Result<T, Response> {
    result.map_err(|e| {
        error!("{}: {}", context, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("error.server.internalError"),
        )
            .into_response()
    })
}

/// Creates a standard internal server error response
///
/// # Arguments
/// * `context` - Context message for logging
/// * `error` - The error that occurred
///
/// # Returns
/// A `Response` with 500 status code
pub fn internal_error<E: std::fmt::Display>(context: &str, error: E) -> Response {
    error!("{}: {}", context, error);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json("error.server.internalError"),
    )
        .into_response()
}

/// Creates a standard unauthorized error response
///
/// # Returns
/// A `Response` with 401 status code
pub fn unauthorized_error() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json("error.user.auth.unauthorized"),
    )
        .into_response()
}

/// Creates a standard not found error response
///
/// # Arguments
/// * `message` - The error message key
///
/// # Returns
/// A `Response` with 404 status code
pub fn not_found_error(message: &str) -> Response {
    (StatusCode::NOT_FOUND, Json(message)).into_response()
}

/// Creates a standard conflict error response
///
/// # Arguments
/// * `message` - The error message key
///
/// # Returns
/// A `Response` with 409 status code
pub fn conflict_error(message: &str) -> Response {
    (StatusCode::CONFLICT, Json(message)).into_response()
}

/// Creates a standard bad request error response
///
/// # Arguments
/// * `message` - The error message key
///
/// # Returns
/// A `Response` with 400 status code
pub fn bad_request_error(message: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(message)).into_response()
}

