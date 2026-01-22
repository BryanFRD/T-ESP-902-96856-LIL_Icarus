use axum::{
    extract::Request,
    extract::State,
    http::{HeaderValue, Method, StatusCode, header::HeaderName},
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies, cookie::time::Duration};

const CSRF_COOKIE_NAME: &str = "csrf_token";
const CSRF_HEADER_NAME: &str = "x-csrf-token";
const CSRF_TOKEN_LENGTH: usize = 32;

pub struct CsrfProtection {
    secret: Arc<String>,
}

impl CsrfProtection {
    pub fn new(secret: String) -> Self {
        Self {
            secret: Arc::new(secret),
        }
    }

    fn generate_token(&self) -> String {
        let random_bytes = Self::generate_random_bytes();
        let mut hasher = Sha256::new();
        hasher.update(random_bytes);
        hasher.update(self.secret.as_bytes());
        let hash = hasher.finalize();
        URL_SAFE_NO_PAD.encode(hash)
    }

    fn generate_random_bytes() -> [u8; CSRF_TOKEN_LENGTH] {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = [0u8; CSRF_TOKEN_LENGTH];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    fn validate_token(cookie_token: &str, header_token: &str) -> bool {
        cookie_token == header_token && !cookie_token.is_empty()
    }

    fn requires_csrf_protection(method: &Method) -> bool {
        matches!(
            method,
            &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE
        )
    }
}

pub async fn csrf_middleware(
    State(csrf): State<Arc<CsrfProtection>>,
    cookies: Cookies,
    req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let csrf_token = if let Some(cookie) = cookies.get(CSRF_COOKIE_NAME) {
        cookie.value().to_string()
    } else {
        let new_token = csrf.generate_token();
        let mut cookie = Cookie::new(CSRF_COOKIE_NAME, new_token.clone());
        cookie.set_path("/");
        cookie.set_http_only(false);
        cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
        cookie.set_max_age(Duration::seconds(60 * 60 * 24));

        if std::env::var("VAULT_ENV").unwrap_or_default() == "prod" {
            cookie.set_domain(".icarus-cyber.fr");
            cookie.set_secure(true);
        }

        cookies.add(cookie);
        new_token
    };

    if CsrfProtection::requires_csrf_protection(&method) {
        let skip_csrf_paths = [
            "/v1/auth/login",
            "/v1/auth/register",
            "/v1/auth/confirm-email",
        ];

        if !skip_csrf_paths.iter().any(|p| path.starts_with(p)) {
            let header_token = req
                .headers()
                .get(CSRF_HEADER_NAME)
                .and_then(|h| h.to_str().ok())
                .unwrap_or("");

            if !CsrfProtection::validate_token(&csrf_token, header_token) {
                tracing::warn!("CSRF token validation failed for {} {}", method, path);
                return Err((StatusCode::FORBIDDEN, "Invalid or missing CSRF token"));
            }
        }
    }

    let mut response = next.run(req).await;

    if std::env::var("VAULT_ENV").unwrap_or_default() != "prod" {
        if let Ok(header_value) = HeaderValue::from_str(&csrf_token) {
            response
                .headers_mut()
                .insert(HeaderName::from_static(CSRF_HEADER_NAME), header_value);
        }
    }

    Ok(response)
}
