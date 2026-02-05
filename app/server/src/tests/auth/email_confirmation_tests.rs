use crate::api::auth::models::EmailConfirmationToken;
use base64::Engine;
use chrono::{Duration, Utc};

#[test]
fn test_email_token_new() {
    let token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john.doe@example.com".to_string(),
        "hashed_password".to_string(),
    );

    assert_eq!(token.username, "testuser");
    assert_eq!(token.first_name, "John");
    assert_eq!(token.last_name, "Doe");
    assert_eq!(token.email, "john.doe@example.com");
    assert_eq!(token.password_hash, "hashed_password");
    assert!(
        !token.is_expired(),
        "Newly created token should not be expired"
    );
}

#[test]
fn test_email_token_encrypt_decrypt() {
    let key = b"test_encryption_key_32_bytes_long";
    let token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "Jane".to_string(),
        "Smith".to_string(),
        "jane.smith@example.com".to_string(),
        "hashed_password_123".to_string(),
    );

    let encrypted = token.encrypt(key).expect("Should encrypt successfully");
    assert!(!encrypted.is_empty(), "Encrypted token should not be empty");

    let decrypted =
        EmailConfirmationToken::decrypt(&encrypted, key).expect("Should decrypt successfully");

    assert_eq!(decrypted.username, token.username);
    assert_eq!(decrypted.first_name, token.first_name);
    assert_eq!(decrypted.last_name, token.last_name);
    assert_eq!(decrypted.email, token.email);
    assert_eq!(decrypted.password_hash, token.password_hash);
}

#[test]
fn test_email_token_decrypt_invalid() {
    let key = b"test_encryption_key";
    let invalid_token = "this_is_not_a_valid_encrypted_token";

    let result = EmailConfirmationToken::decrypt(invalid_token, key);
    assert!(result.is_err(), "Decrypting invalid token should fail");
}

#[test]
fn test_email_token_is_expired() {
    let mut token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "hash".to_string(),
    );

    token.expires_at = Utc::now() - Duration::minutes(1);

    assert!(
        token.is_expired(),
        "Token with past expiry should be expired"
    );
}

#[test]
fn test_email_token_not_expired() {
    let token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "hash".to_string(),
    );

    assert!(
        !token.is_expired(),
        "Freshly created token should not be expired"
    );
}

#[test]
fn test_email_token_decrypt_wrong_key() {
    let correct_key = b"correct_key_here";
    let wrong_key = b"wrong_key_here";

    let token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "hash".to_string(),
    );

    let encrypted = token.encrypt(correct_key).expect("Should encrypt");

    let result = EmailConfirmationToken::decrypt(&encrypted, wrong_key);
    assert!(result.is_err(), "Decrypting with wrong key should fail");
}

#[test]
fn test_email_token_decrypt_expired() {
    let key = b"test_key";
    let mut token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "hash".to_string(),
    );

    token.expires_at = Utc::now() - Duration::minutes(1);

    let encrypted = token.encrypt(key).expect("Should encrypt");

    let result = EmailConfirmationToken::decrypt(&encrypted, key);
    assert!(result.is_err(), "Decrypting expired token should fail");

    if let Err(e) = result {
        assert!(
            e.to_string().contains("expired"),
            "Error should indicate token expired"
        );
    }
}

#[test]
fn test_email_token_empty_string_decrypt() {
    let key = b"test_key";
    let result = EmailConfirmationToken::decrypt("", key);
    assert!(result.is_err(), "Decrypting empty string should fail");
}

#[test]
fn test_email_token_special_characters() {
    let key = b"test_key";
    let token = EmailConfirmationToken::new(
        "user@#$%".to_string(),
        "Jöhn".to_string(),
        "Døe".to_string(),
        "test+email@example.com".to_string(),
        "hash!@#$%^&*()".to_string(),
    );

    let encrypted = token.encrypt(key).expect("Should encrypt");
    let decrypted = EmailConfirmationToken::decrypt(&encrypted, key).expect("Should decrypt");

    assert_eq!(decrypted.username, "user@#$%");
    assert_eq!(decrypted.first_name, "Jöhn");
    assert_eq!(decrypted.last_name, "Døe");
    assert_eq!(decrypted.email, "test+email@example.com");
}

#[test]
fn test_email_token_encryption_different_each_time() {
    let key = b"test_key";
    let token = EmailConfirmationToken::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "hash".to_string(),
    );

    let encrypted1 = token.encrypt(key).expect("Should encrypt");
    let encrypted2 = token.encrypt(key).expect("Should encrypt");

    assert_ne!(
        encrypted1, encrypted2,
        "Encryption should use random nonces, producing different ciphertext each time"
    );
}

#[test]
fn test_email_token_decrypt_too_short() {
    let key = b"test_key";
    // Create a valid base64 string but with less than 12 bytes
    let short_data = vec![1u8, 2, 3, 4, 5]; // Only 5 bytes
    let short_token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&short_data);

    let result = EmailConfirmationToken::decrypt(&short_token, key);
    assert!(result.is_err(), "Token shorter than 12 bytes should fail");

    if let Err(e) = result {
        assert!(
            e.to_string().contains("Token too short") || e.to_string().contains("decrypt"),
            "Error should indicate token is too short"
        );
    }
}

#[test]
fn test_email_token_decrypt_exact_12_bytes() {
    let key = b"test_key";
    // Create exactly 12 bytes (nonce size) - should fail because no ciphertext
    let exact_12_bytes = vec![0u8; 12];
    let token_12 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&exact_12_bytes);

    let result = EmailConfirmationToken::decrypt(&token_12, key);
    // This should fail during decryption because there's no ciphertext
    assert!(
        result.is_err(),
        "Token with only nonce (no ciphertext) should fail"
    );
}

#[test]
fn test_email_token_decrypt_corrupted_ciphertext() {
    let key = b"test_key";
    // Create a token with 12-byte nonce + some random bytes (not valid ciphertext)
    let mut corrupted = vec![0u8; 12]; // nonce
    corrupted.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]); // invalid ciphertext
    let corrupted_token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&corrupted);

    let result = EmailConfirmationToken::decrypt(&corrupted_token, key);
    assert!(
        result.is_err(),
        "Token with corrupted ciphertext should fail"
    );
}
