use std::sync::Arc;

use axum::http::{HeaderName, HeaderValue};
use axum::{Extension, Json};
use serde::Serialize;
use tower_cookies::Cookies;

use crate::middlewares::CsrfProtection;

const CSRF_HEADER_NAME: &str = "x-csrf-token";

#[derive(Serialize)]
pub struct CsrfResponse {
    pub token: String,
}

pub async fn get_csrf(
    Extension(csrf): Extension<Arc<CsrfProtection>>,
    cookies: Cookies,
) -> (axum::http::HeaderMap, axum::Json<CsrfResponse>) {
    let token = csrf.ensure_token_cookie(&cookies);

    let mut headers = axum::http::HeaderMap::new();

    if std::env::var("VAULT_ENV").unwrap_or_default() != "prod"
        && let Ok(header_value) = HeaderValue::from_str(&token)
    {
        headers.insert(HeaderName::from_static(CSRF_HEADER_NAME), header_value);
    }

    (headers, Json(CsrfResponse { token }))
}
