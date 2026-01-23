use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use tower_cookies::Cookies;

use crate::middlewares::CsrfProtection;

#[derive(Serialize, utoipa::ToSchema)]
pub struct CsrfResponse {
    pub token: String,
}

#[utoipa::path(
    get,
    path = "/v1/csrf",
    description = "Get or refresh CSRF token",
    responses(
        (status = 200, description = "CSRF token retrieved successfully", body = CsrfResponse),
    ),
    tag = "Auth"
)]
pub async fn get_csrf(
    Extension(csrf): Extension<Arc<CsrfProtection>>,
    cookies: Cookies,
) -> Json<CsrfResponse> {
    let token = csrf.ensure_token_cookie(&cookies);
    Json(CsrfResponse { token })
}
