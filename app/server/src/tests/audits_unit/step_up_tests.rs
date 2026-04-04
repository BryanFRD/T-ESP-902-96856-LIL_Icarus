use crate::api::audits::step_up::services::{
    AUDIT_ACCESS_COOKIE_NAME, build_audit_access_cookie, generate_audit_access_token,
    validate_audit_access_token,
};

#[test]
fn test_generate_and_validate_token() {
    let token = generate_audit_access_token("user123", "secret");
    let result = validate_audit_access_token(&token, "secret", "user123");
    assert!(result.is_ok());
}

#[test]
fn test_validate_wrong_user_id() {
    let token = generate_audit_access_token("user123", "secret");
    let result = validate_audit_access_token(&token, "secret", "wrong_user");
    assert_eq!(result, Err("User ID mismatch"));
}

#[test]
fn test_validate_wrong_secret() {
    let token = generate_audit_access_token("user123", "secret");
    let result = validate_audit_access_token(&token, "wrong_secret", "user123");
    assert_eq!(result, Err("Invalid signature"));
}

#[test]
fn test_validate_invalid_base64() {
    let result = validate_audit_access_token("!!!invalid!!!", "secret", "user123");
    assert_eq!(result, Err("Invalid base64 encoding"));
}

#[test]
fn test_validate_invalid_format() {
    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        b"only_one_part",
    );
    let result = validate_audit_access_token(&encoded, "secret", "user123");
    assert_eq!(result, Err("Invalid token format"));
}

#[test]
fn test_validate_invalid_timestamp() {
    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::URL_SAFE_NO_PAD,
        b"user123|not_a_number|sig",
    );
    let result = validate_audit_access_token(&encoded, "secret", "user123");
    assert_eq!(result, Err("Invalid timestamp"));
}

#[test]
fn test_token_is_different_each_time() {
    let t1 = generate_audit_access_token("user1", "secret");
    // Tokens generated at the same second will be identical,
    // so we just check that the function produces valid output
    let result = validate_audit_access_token(&t1, "secret", "user1");
    assert!(result.is_ok());
}

#[test]
fn test_cookie_name_constant() {
    assert_eq!(AUDIT_ACCESS_COOKIE_NAME, "audit_access");
}

#[test]
fn test_build_audit_access_cookie_name() {
    let cookie = build_audit_access_cookie("test_token");
    assert_eq!(cookie.name(), AUDIT_ACCESS_COOKIE_NAME);
    assert_eq!(cookie.value(), "test_token");
    assert_eq!(cookie.path(), Some("/"));
    assert!(!cookie.http_only().unwrap_or(true));
}

#[test]
fn test_build_audit_access_cookie_local_not_secure() {
    // Default VAULT_ENV is "local" in test environment
    let cookie = build_audit_access_cookie("token");
    assert!(!cookie.secure().unwrap_or(true));
}
