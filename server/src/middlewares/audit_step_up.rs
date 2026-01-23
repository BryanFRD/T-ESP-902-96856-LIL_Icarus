use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tower_cookies::Cookies;

use crate::api::auth::models::JwtClaims;
use crate::api::audits::step_up::services::{validate_audit_access_token, AUDIT_ACCESS_COOKIE_NAME};
use crate::state::GlobalState;

/// Middleware to enforce audit access step-up authentication
///
/// This middleware should be applied to all audit routes that require
/// the user to have verified their password recently.
pub async fn audit_step_up_middleware(
    State(state): State<GlobalState>,
    jwt_claims: JwtClaims,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let user_id = jwt_claims.sub.to_string();

    let audit_cookie = cookies.get(AUDIT_ACCESS_COOKIE_NAME);

    if let Some(cookie) = audit_cookie {
        match validate_audit_access_token(cookie.value(), &state.audit_access_secret, &user_id) {
            Ok(()) => {
                return Ok(next.run(req).await);
            }
            Err(reason) => {
                tracing::warn!("Audit access denied for user {}: {}", user_id, reason);
                metrics::counter!("audit_access_denied_total", "reason" => "invalid").increment(1);

                return Err((
                    StatusCode::FORBIDDEN,
                    axum::Json(json!({
                        "error": "error.audit.stepUp.invalid",
                        "code": "AUDIT_STEP_UP_INVALID"
                    })),
                ));
            }
        }
    }

    tracing::warn!("Audit access denied for user {}: missing token", user_id);
    metrics::counter!("audit_access_denied_total", "reason" => "missing").increment(1);

    Err((
        StatusCode::FORBIDDEN,
        axum::Json(json!({
            "error": "error.audit.stepUp.required",
            "code": "AUDIT_STEP_UP_REQUIRED"
        })),
    ))
}
