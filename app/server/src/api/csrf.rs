use std::sync::Arc;

use axum::{Extension, Json};
use serde::Serialize;
use tower_cookies::Cookies;

use crate::middlewares::CsrfProtection;

#[derive(Serialize)]
pub struct CsrfResponse {
    pub token: String,
}

pub async fn get_csrf(
    Extension(csrf): Extension<Arc<CsrfProtection>>,
    cookies: Cookies,
) -> Json<CsrfResponse> {
    let token = csrf.ensure_token_cookie(&cookies);
    Json(CsrfResponse { token })
}
