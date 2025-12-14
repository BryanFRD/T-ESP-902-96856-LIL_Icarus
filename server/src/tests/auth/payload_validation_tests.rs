use crate::api::auth::{
    ConfirmEmailPayload, LoginPayload, RegisterPayload, ResetPasswordPayload,
    ResetPasswordRequestPayload,
};
use validator::Validate;

// ============================================================================
// LoginPayload Tests (5 tests)
// ============================================================================

#[test]
fn test_login_payload_valid_email() {
    let payload = LoginPayload {
        email: "user@example.com".to_string(),
        password: "anypassword".to_string(),
    };

    assert!(
        payload.validate().is_ok(),
        "Valid email should pass validation"
    );
}

#[test]
fn test_login_payload_invalid_email_format() {
    let payload = LoginPayload {
        email: "not-an-email".to_string(),
        password: "anypassword".to_string(),
    };

    assert!(
        payload.validate().is_err(),
        "Invalid email format should fail validation"
    );
}

#[test]
fn test_login_payload_empty_email() {
    let payload = LoginPayload {
        email: String::new(),
        password: "anypassword".to_string(),
    };

    assert!(
        payload.validate().is_err(),
        "Empty email should fail validation"
    );
}

#[test]
fn test_login_payload_accepts_any_password() {
    let payload = LoginPayload {
        email: "user@example.com".to_string(),
        password: String::new(),
    };

    // Password has no validation, so empty password should be OK
    assert!(
        payload.validate().is_ok(),
        "Any password (even empty) should pass since there's no password validation"
    );
}

#[test]
fn test_login_payload_special_chars_in_email() {
    let payload = LoginPayload {
        email: "user+tag@example.com".to_string(),
        password: "password123".to_string(),
    };

    assert!(
        payload.validate().is_ok(),
        "Email with + sign should be valid"
    );
}

// ============================================================================
// RegisterPayload Tests (5 tests)
// ============================================================================

#[test]
fn test_register_payload_valid() {
    let payload = RegisterPayload {
        username: "johndoe".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "password123".to_string(),
    };

    assert!(
        payload.validate().is_ok(),
        "Valid RegisterPayload should pass validation"
    );
}

#[test]
fn test_register_payload_invalid_email() {
    let payload = RegisterPayload {
        username: "johndoe".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "not-an-email".to_string(),
        password: "password123".to_string(),
    };

    let result = payload.validate();
    assert!(result.is_err(), "Invalid email should fail validation");

    // Verify the error message
    if let Err(ref errors) = result {
        let field_errors = errors.field_errors();
        let email_errors = field_errors.get("email");
        assert!(email_errors.is_some(), "Should have email field error");
    }
}

#[test]
fn test_register_payload_empty_username() {
    let payload = RegisterPayload {
        username: String::new(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "password123".to_string(),
    };

    // No validation on username length, so empty should be OK
    assert!(
        payload.validate().is_ok(),
        "Empty username should pass since there's no length validation"
    );
}

#[test]
fn test_register_payload_empty_password() {
    let payload = RegisterPayload {
        username: "johndoe".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        password: String::new(),
    };

    // No validation on password, so empty should be OK
    assert!(
        payload.validate().is_ok(),
        "Empty password should pass since there's no password validation"
    );
}

#[test]
fn test_register_payload_unicode_in_names() {
    let payload = RegisterPayload {
        username: "jose".to_string(),
        first_name: "José".to_string(),
        last_name: "García".to_string(),
        email: "jose@example.com".to_string(),
        password: "password123".to_string(),
    };

    assert!(
        payload.validate().is_ok(),
        "Unicode characters in names should be valid"
    );
}

// ============================================================================
// ResetPasswordRequestPayload Tests (2 tests)
// ============================================================================

#[test]
fn test_reset_request_payload_valid() {
    let payload = ResetPasswordRequestPayload {
        reset_email: "user@example.com".to_string(),
    };

    // No validation on this struct, construction should always succeed
    assert_eq!(payload.reset_email, "user@example.com");
}

#[test]
fn test_reset_request_payload_accepts_invalid_email() {
    let payload = ResetPasswordRequestPayload {
        reset_email: "not-an-email".to_string(),
    };

    // No validation - security by obscurity (don't reveal if email exists)
    assert_eq!(payload.reset_email, "not-an-email");
}

// ============================================================================
// ResetPasswordPayload Tests (3 tests)
// ============================================================================

#[test]
fn test_reset_password_payload_valid() {
    let payload = ResetPasswordPayload {
        token: "valid_token_string".to_string(),
        new_password: "newpassword123".to_string(),
    };

    // No validation, construction should succeed
    assert_eq!(payload.token, "valid_token_string");
    assert_eq!(payload.new_password, "newpassword123");
}

#[test]
fn test_reset_password_payload_empty_token() {
    let payload = ResetPasswordPayload {
        token: String::new(),
        new_password: "newpassword123".to_string(),
    };

    // No validation on struct - validation happens in endpoint
    assert_eq!(payload.token, "");
}

#[test]
fn test_reset_password_payload_empty_password() {
    let payload = ResetPasswordPayload {
        token: "valid_token".to_string(),
        new_password: String::new(),
    };

    // No validation on struct - validation happens in endpoint
    assert_eq!(payload.new_password, "");
}

// ============================================================================
// ConfirmEmailPayload Tests (2 tests)
// ============================================================================

#[test]
fn test_confirm_email_payload_valid() {
    let payload = ConfirmEmailPayload {
        token: "AbCdEfGhIjKlMnOpQrStUvWxYz0123456789-_".to_string(),
    };

    // No validation, construction should succeed
    assert_eq!(payload.token, "AbCdEfGhIjKlMnOpQrStUvWxYz0123456789-_");
}

#[test]
fn test_confirm_email_payload_serialization() {
    let payload = ConfirmEmailPayload {
        token: "test_token_123".to_string(),
    };

    // Test serialization round-trip
    let json = serde_json::to_string(&payload).expect("Should serialize");
    let deserialized: ConfirmEmailPayload =
        serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(payload.token, deserialized.token);
}

// ============================================================================
// General Derive Tests (1 test)
// ============================================================================

#[test]
fn test_all_payloads_derive_clone_debug() {
    // Test that all payloads implement Clone and Debug
    let login = LoginPayload {
        email: "test@example.com".to_string(),
        password: "pass".to_string(),
    };
    let login_clone = login.clone();
    assert_eq!(login.email, login_clone.email);
    let _debug_str = format!("{login:?}");

    let register = RegisterPayload {
        username: "user".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
        email: "test@example.com".to_string(),
        password: "pass".to_string(),
    };
    let register_clone = register.clone();
    assert_eq!(register.username, register_clone.username);
    let _debug_str = format!("{register:?}");

    let reset_request = ResetPasswordRequestPayload {
        reset_email: "test@example.com".to_string(),
    };
    let reset_request_clone = reset_request.clone();
    assert_eq!(reset_request.reset_email, reset_request_clone.reset_email);
    let _debug_str = format!("{reset_request:?}");

    let reset = ResetPasswordPayload {
        token: "token".to_string(),
        new_password: "newpass".to_string(),
    };
    let reset_clone = reset.clone();
    assert_eq!(reset.token, reset_clone.token);
    let _debug_str = format!("{reset:?}");

    let confirm = ConfirmEmailPayload {
        token: "token".to_string(),
    };
    let confirm_clone = confirm.clone();
    assert_eq!(confirm.token, confirm_clone.token);
    let _debug_str = format!("{confirm:?}");
}
