use crate::api::ws::rooms::permissions::validate_step_up_from_headers;
use http::HeaderMap;

#[test]
fn test_step_up_bypass_in_test_mode() {
    let headers = HeaderMap::new();
    // When secret contains "test", should bypass step-up
    let result = validate_step_up_from_headers(&headers, "my_test_secret", "user1");
    assert!(result.is_ok());
}

#[test]
fn test_step_up_missing_cookie_header() {
    let headers = HeaderMap::new();
    let result = validate_step_up_from_headers(&headers, "production_secret", "user1");
    assert!(result.is_err());
}

#[test]
fn test_step_up_cookie_header_without_audit_access() {
    let mut headers = HeaderMap::new();
    headers.insert(http::header::COOKIE, "auth=jwt_token; other=val".parse().unwrap());
    let result = validate_step_up_from_headers(&headers, "production_secret", "user1");
    assert!(result.is_err());
}

#[test]
fn test_step_up_invalid_audit_access_token() {
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::COOKIE,
        "audit_access=invalid_token".parse().unwrap(),
    );
    let result = validate_step_up_from_headers(&headers, "production_secret", "user1");
    assert!(result.is_err());
}
