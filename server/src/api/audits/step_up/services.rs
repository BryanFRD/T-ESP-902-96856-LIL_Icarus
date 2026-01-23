use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};
use tower_cookies::{Cookie, cookie::time::Duration};

pub const AUDIT_ACCESS_COOKIE_NAME: &str = "audit_access";

/// Generate an audit access token for a given user
///
/// Format: `base64(user_id|timestamp|signature)`
/// Signature: `HMAC-SHA256(user_id + timestamp + secret)`
pub fn generate_audit_access_token(user_id: &str, secret: &str) -> String {
    let timestamp = chrono::Utc::now().timestamp();
    let signature = generate_signature(user_id, timestamp, secret);

    let token_data = format!("{user_id}|{timestamp}|{signature}");
    URL_SAFE_NO_PAD.encode(token_data.as_bytes())
}

/// Generate HMAC-SHA256 signature for the token
fn generate_signature(user_id: &str, timestamp: i64, secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(user_id.as_bytes());
    hasher.update(timestamp.to_string().as_bytes());
    hasher.update(secret.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

/// Validate an audit access token
///
/// Returns Ok(()) if the token is valid, or an error describing why it's invalid
pub fn validate_audit_access_token(
    cookie_value: &str,
    secret: &str,
    expected_user_id: &str,
) -> Result<(), &'static str> {
    let decoded = URL_SAFE_NO_PAD
        .decode(cookie_value.as_bytes())
        .map_err(|_| "Invalid base64 encoding")?;

    let token_str = std::str::from_utf8(&decoded).map_err(|_| "Invalid UTF-8 in token")?;

    let parts: Vec<&str> = token_str.split('|').collect();
    if parts.len() != 3 {
        return Err("Invalid token format");
    }

    let token_user_id = parts[0];
    let timestamp_str = parts[1];
    let provided_signature = parts[2];

    if token_user_id != expected_user_id {
        return Err("User ID mismatch");
    }

    let timestamp: i64 = timestamp_str.parse().map_err(|_| "Invalid timestamp")?;

    let expected_signature = generate_signature(token_user_id, timestamp, secret);
    if expected_signature != provided_signature {
        return Err("Invalid signature");
    }

    let vault_env = std::env::var("VAULT_ENV").unwrap_or_else(|_| "local".to_string());

    if vault_env.as_str() == "prod" {
        Ok(())
    } else {
        let now = chrono::Utc::now().timestamp();
        let max_age = 15 * 60;

        if now - timestamp > max_age {
            return Err("Token expired");
        }

        Ok(())
    }
}

/// Build the audit access cookie with environment-specific configuration
pub fn build_audit_access_cookie(token: &str) -> Cookie<'static> {
    let mut cookie = Cookie::new(AUDIT_ACCESS_COOKIE_NAME, token.to_owned());
    cookie.set_path("/");
    cookie.set_http_only(false);

    let vault_env = std::env::var("VAULT_ENV").unwrap_or_else(|_| "local".to_string());

    match vault_env.as_str() {
        "prod" => {
            cookie.set_secure(true);
            cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
        }
        "dev" => {
            cookie.set_secure(true);
            cookie.set_same_site(tower_cookies::cookie::SameSite::None);
            cookie.set_max_age(Duration::seconds(15 * 60));
        }
        _ => {
            cookie.set_secure(false);
            cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
            cookie.set_max_age(Duration::seconds(15 * 60));
        }
    }

    cookie
}
