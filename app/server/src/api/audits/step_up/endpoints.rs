use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_cookies::Cookies;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    api::auth::JwtClaims, api::users::get_user_by_id, helpers::internal_error, state::GlobalState,
};

use super::services::{
    AUDIT_ACCESS_COOKIE_NAME, build_audit_access_cookie, generate_audit_access_token,
    validate_audit_access_token,
};

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct VerifyAccessPayload {
    #[validate(length(min = 1))]
    #[schema(example = "my_secure_password")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckAccessResponse {
    pub has_access: bool,
    pub expires_at: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/audits/verify-access",
    tag = "Audits",
    request_body = VerifyAccessPayload,
    responses(
        (status = 200, description = "Access granted, cookie set"),
        (status = 401, description = "Invalid password or unauthorized"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn verify_access(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    cookies: Cookies,
    Json(payload): Json<VerifyAccessPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": format!("Validation error: {}", e)
            })),
        )
            .into_response();
    }

    let user_id = jwt_user.sub.to_string();

    let user = match get_user_by_id(&state, &user_id) {
        Ok(Some(u)) => u,
        Ok(None) => {
            tracing::warn!("User not found: {}", user_id);
            metrics::counter!("audit_step_up_total", "status" => "failure").increment(1);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "error.audit.stepUp.userNotFound"
                })),
            )
                .into_response();
        }
        Err(e) => {
            return internal_error("Failed to get user", e);
        }
    };

    if !user.verify_password(&payload.password) {
        tracing::warn!("Invalid password for user: {}", user_id);
        metrics::counter!("audit_step_up_total", "status" => "failure").increment(1);
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "error.audit.stepUp.invalidPassword"
            })),
        )
            .into_response();
    }

    let token = generate_audit_access_token(&user_id, &state.audit_access_secret);

    let cookie = build_audit_access_cookie(&token);
    cookies.add(cookie);

    tracing::info!("Audit access granted for user: {}", user_id);
    metrics::counter!("audit_step_up_total", "status" => "success").increment(1);

    (
        StatusCode::OK,
        Json(json!({
            "message": "success.audit.stepUp.verified"
        })),
    )
        .into_response()
}

#[utoipa::path(
    get,
    path = "/v1/audits/check-access",
    tag = "Audits",
    responses(
        (status = 200, description = "Access status", body = CheckAccessResponse),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn check_access(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    cookies: Cookies,
) -> impl IntoResponse {
    let user_id = jwt_user.sub.to_string();

    let has_access = cookies
        .get(AUDIT_ACCESS_COOKIE_NAME)
        .and_then(|cookie| {
            validate_audit_access_token(cookie.value(), &state.audit_access_secret, &user_id).ok()
        })
        .is_some();

    let expires_at = if has_access {
        calculate_expiration()
    } else {
        None
    };

    Json(CheckAccessResponse {
        has_access,
        expires_at,
    })
    .into_response()
}

/// Calculate when the audit access token will expire
/// Returns None for session cookies (prod), or ISO 8601 timestamp for dev/local
fn calculate_expiration() -> Option<String> {
    let vault_env = std::env::var("VAULT_ENV").unwrap_or_else(|_| "local".to_string());

    if vault_env.as_str() == "prod" {
        None
    } else {
        let expiration = chrono::Utc::now() + chrono::Duration::minutes(15);
        Some(expiration.to_rfc3339())
    }
}
