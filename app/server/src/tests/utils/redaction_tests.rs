use crate::utils::redaction::{redact_email, redact_json_value, redact_string};
use serde_json::json;

#[test]
fn test_redact_string_empty() {
    let result = redact_string("");
    assert_eq!(result, "");
}

#[test]
fn test_redact_string_single_char() {
    let result = redact_string("a");
    assert_eq!(result, "x");
}

#[test]
fn test_redact_string_two_chars() {
    let result = redact_string("ab");
    assert_eq!(result, "xx");
}

#[test]
fn test_redact_string_multiple_chars() {
    let result = redact_string("password");
    assert_eq!(result, "pxxxxd");
    assert_eq!(result.len(), 6);
}

#[test]
fn test_redact_string_unicode() {
    let result = redact_string("héllo");
    assert_eq!(result.chars().count(), 6);
    assert!(result.starts_with('h'));
    assert!(result.ends_with('o'));
}

#[test]
fn test_redact_email_valid() {
    let result = redact_email("john.doe@example.com");
    assert!(result.contains("@example.com"));
    assert!(result.starts_with('j'));
    assert!(result.contains("xxxx"));
    assert!(!result.contains("john.doe"));
}

#[test]
fn test_redact_email_short_local() {
    let result = redact_email("a@example.com");
    assert_eq!(result, "x@example.com");
}

#[test]
fn test_redact_email_two_char_local() {
    let result = redact_email("ab@example.com");
    assert!(result.starts_with('a'));
    assert!(result.contains("@example.com"));
}

#[test]
fn test_redact_email_invalid_no_at() {
    let result = redact_email("notanemail");
    assert_eq!(result, "nxxxxl");
}

#[test]
fn test_redact_json_always_hidden_fields() {
    let mut json = json!({
        "username": "john",
        "password": "secret123",
        "token": "abc123",
        "api_key": "key123",
        "secret": "mysecret",
        "jwt": "jwt.token.here"
    });

    redact_json_value(&mut json, true);

    assert_eq!(json["username"], "john");
    assert_eq!(json["password"], "HIDDEN");
    assert_eq!(json["token"], "HIDDEN");
    assert_eq!(json["api_key"], "HIDDEN");
    assert_eq!(json["secret"], "HIDDEN");
    assert_eq!(json["jwt"], "HIDDEN");
}

#[test]
fn test_redact_json_non_admin_redacts_personal_info() {
    let mut json = json!({
        "username": "johndoe",
        "email": "john@example.com",
        "name": "John Doe",
        "first_name": "John",
        "last_name": "Doe"
    });

    redact_json_value(&mut json, false);

    assert_eq!(json["username"].as_str().unwrap(), "jxxxxe");
    assert!(json["email"].as_str().unwrap().contains("xxxx"));
    assert!(json["name"].as_str().unwrap().contains("xxxx"));
    assert!(json["first_name"].as_str().unwrap().contains("xxxx"));
    assert!(json["last_name"].as_str().unwrap().contains("xxxx"));
}

#[test]
fn test_redact_json_admin_keeps_personal_info() {
    let mut json = json!({
        "username": "johndoe",
        "email": "john@example.com",
        "name": "John Doe"
    });

    let original_json = json.clone();
    redact_json_value(&mut json, true);

    assert_eq!(json["username"], original_json["username"]);
    assert_eq!(json["email"], original_json["email"]);
    assert_eq!(json["name"], original_json["name"]);
}

#[test]
fn test_redact_json_nested_objects() {
    let mut json = json!({
        "user": {
            "password": "secret",
            "email": "test@example.com",
            "profile": {
                "api_key": "key123"
            }
        }
    });

    redact_json_value(&mut json, false);

    assert_eq!(json["user"]["password"], "HIDDEN");
    assert_eq!(json["user"]["profile"]["api_key"], "HIDDEN");
}

#[test]
fn test_redact_json_arrays() {
    let mut json = json!({
        "users": [
            {"email": "user1@example.com", "password": "pass1"},
            {"email": "user2@example.com", "password": "pass2"}
        ]
    });

    redact_json_value(&mut json, false);

    assert_eq!(json["users"][0]["password"], "HIDDEN");
    assert_eq!(json["users"][1]["password"], "HIDDEN");
    assert!(json["users"][0]["email"].as_str().unwrap().contains("xxxx"));
}

#[test]
fn test_redact_json_preserves_non_string_values() {
    let mut json = json!({
        "count": 42,
        "active": true,
        "ratio": 3.14,
        "items": null
    });

    redact_json_value(&mut json, false);

    assert_eq!(json["count"], 42);
    assert_eq!(json["active"], true);
    assert_eq!(json["ratio"], 3.14);
    assert_eq!(json["items"], json!(null));
}

#[test]
fn test_redact_json_empty_object() {
    let mut json = json!({});
    redact_json_value(&mut json, false);
    assert_eq!(json, json!({}));
}

#[test]
fn test_redact_json_empty_array() {
    let mut json = json!([]);
    redact_json_value(&mut json, false);
    assert_eq!(json, json!([]));
}
