use crate::api::auth::models::PasswordResetToken;
use chrono::{Duration, Utc};

#[test]
fn test_password_reset_token_is_valid() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: Utc::now().naive_utc(),
    };

    assert!(token.is_valid(), "Freshly created token should be valid");
    assert!(
        !token.is_expired(),
        "Freshly created token should not be expired"
    );
}

#[test]
fn test_password_reset_token_is_expired() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: (Utc::now() - Duration::minutes(20)).naive_utc(),
    };

    assert!(
        token.is_expired(),
        "Token created 20 minutes ago should be expired"
    );
    assert!(!token.is_valid(), "Expired token should not be valid");
}

#[test]
fn test_password_reset_token_edge_case_exact_15min() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: (Utc::now() - Duration::minutes(15)).naive_utc(),
    };

    assert!(
        token.is_expired(),
        "Token at exactly 15 minutes should be expired"
    );
}

#[test]
fn test_password_reset_token_just_before_expiry() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: (Utc::now() - Duration::minutes(14) - Duration::seconds(59)).naive_utc(),
    };

    assert!(
        !token.is_expired(),
        "Token just before 15 minutes should not be expired"
    );
    assert!(
        token.is_valid(),
        "Token just before 15 minutes should be valid"
    );
}

#[test]
fn test_password_reset_token_just_after_expiry() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: (Utc::now() - Duration::minutes(15) - Duration::seconds(1)).naive_utc(),
    };

    assert!(
        token.is_expired(),
        "Token just after 15 minutes should be expired"
    );
    assert!(
        !token.is_valid(),
        "Token just after 15 minutes should not be valid"
    );
}

#[test]
fn test_password_reset_token_very_old() {
    let token = PasswordResetToken {
        user_id: "test_user_id".to_string(),
        token_hash: "test_hash".to_string(),
        created_at: (Utc::now() - Duration::days(1)).naive_utc(),
    };

    assert!(
        token.is_expired(),
        "Token created 1 day ago should be expired"
    );
    assert!(!token.is_valid(), "Very old token should not be valid");
}
