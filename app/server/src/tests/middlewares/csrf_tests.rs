use crate::middlewares::CsrfProtection;
use serial_test::serial;

#[test]
#[serial]
fn test_csrf_cookie_local() {
    unsafe {
        std::env::set_var("VAULT_ENV", "local");
    }
    let cookie = CsrfProtection::build_cookie("test_token");
    assert_eq!(
        cookie.secure(),
        Some(false),
        "Local cookie should not be secure"
    );
    assert_eq!(
        cookie.same_site(),
        Some(tower_cookies::cookie::SameSite::Lax),
        "Local cookie should use SameSite=Lax"
    );
    assert_eq!(cookie.http_only(), Some(true), "Cookie should be HttpOnly");
    assert_eq!(
        cookie.domain(),
        None,
        "Cookie should not set explicit domain"
    );
}

#[test]
#[serial]
fn test_csrf_cookie_dev() {
    unsafe {
        std::env::set_var("VAULT_ENV", "dev");
    }
    let cookie = CsrfProtection::build_cookie("test_token");
    assert_eq!(cookie.secure(), Some(true), "Dev cookie should be secure");
    assert_eq!(
        cookie.same_site(),
        Some(tower_cookies::cookie::SameSite::None),
        "Dev cookie should use SameSite=None for cross-origin"
    );
    assert_eq!(cookie.http_only(), Some(true), "Cookie should be HttpOnly");
    assert_eq!(
        cookie.domain(),
        None,
        "Cookie should not set explicit domain"
    );
}

#[test]
#[serial]
fn test_csrf_cookie_prod() {
    unsafe {
        std::env::set_var("VAULT_ENV", "prod");
    }
    let cookie = CsrfProtection::build_cookie("test_token");
    assert_eq!(cookie.secure(), Some(true), "Prod cookie should be secure");
    assert_eq!(
        cookie.same_site(),
        Some(tower_cookies::cookie::SameSite::Lax),
        "Prod cookie should use SameSite=Lax"
    );
    assert_eq!(cookie.http_only(), Some(true), "Cookie should be HttpOnly");
    assert_eq!(
        cookie.domain(),
        None,
        "Cookie should not set explicit domain"
    );
}

#[test]
fn test_requires_csrf_protection() {
    use axum::http::Method;

    assert!(CsrfProtection::requires_csrf_protection(&Method::POST));
    assert!(CsrfProtection::requires_csrf_protection(&Method::PUT));
    assert!(CsrfProtection::requires_csrf_protection(&Method::PATCH));
    assert!(CsrfProtection::requires_csrf_protection(&Method::DELETE));
    assert!(!CsrfProtection::requires_csrf_protection(&Method::GET));
    assert!(!CsrfProtection::requires_csrf_protection(&Method::HEAD));
    assert!(!CsrfProtection::requires_csrf_protection(&Method::OPTIONS));
}

#[test]
fn test_validate_token() {
    assert!(CsrfProtection::validate_token("token123", "token123"));
    assert!(!CsrfProtection::validate_token("token123", "different"));
    assert!(!CsrfProtection::validate_token("", ""));
    assert!(!CsrfProtection::validate_token("token123", ""));
}

#[test]
#[serial]
fn test_csrf_cookie_path_is_root() {
    unsafe {
        std::env::set_var("VAULT_ENV", "local");
    }
    let cookie = CsrfProtection::build_cookie("value");
    assert_eq!(cookie.path(), Some("/"));
    assert_eq!(cookie.name(), "csrf_token");
    assert_eq!(cookie.value(), "value");
}

#[test]
#[serial]
fn test_csrf_cookie_max_age() {
    unsafe {
        std::env::set_var("VAULT_ENV", "local");
    }
    let cookie = CsrfProtection::build_cookie("tok");
    let max_age = cookie.max_age().expect("max_age should be set");
    // 24 hours = 86400 seconds
    assert_eq!(max_age.whole_seconds(), 86400);
}
