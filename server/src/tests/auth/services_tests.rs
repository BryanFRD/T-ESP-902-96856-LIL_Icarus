use crate::api::auth::hash_password;

#[test]
fn test_hash_password_deterministic() {
    let password = "test_password_123";
    let hash1 = hash_password(password);
    let hash2 = hash_password(password);

    assert_eq!(hash1, hash2, "Same password should produce same hash");
    assert_eq!(hash1.len(), 64, "SHA-256 hash should be 64 hex characters");
}

#[test]
fn test_hash_password_different_inputs() {
    let password1 = "password1";
    let password2 = "password2";

    let hash1 = hash_password(password1);
    let hash2 = hash_password(password2);

    assert_ne!(
        hash1, hash2,
        "Different passwords should produce different hashes"
    );
}

#[test]
fn test_hash_password_empty_string() {
    let password = "";
    let hash = hash_password(password);

    assert_eq!(
        hash.len(),
        64,
        "Hash of empty string should still be 64 characters"
    );
    assert!(!hash.is_empty(), "Hash should not be empty");
}

#[test]
fn test_hash_password_special_characters() {
    let password = "p@ssw0rd!#$%^&*()_+-=[]{}|;':\",./<>?";
    let hash = hash_password(password);

    assert_eq!(
        hash.len(),
        64,
        "Hash with special characters should be 64 characters"
    );

    let hash2 = hash_password(password);
    assert_eq!(
        hash, hash2,
        "Hash should be deterministic even with special characters"
    );
}

#[test]
fn test_hash_password_unicode() {
    let password = "пароль密码🔒";
    let hash = hash_password(password);

    assert_eq!(hash.len(), 64, "Hash with unicode should be 64 characters");

    let hash2 = hash_password(password);
    assert_eq!(hash, hash2, "Hash should be deterministic with unicode");
}

#[test]
fn test_hash_password_long_input() {
    let password = "a".repeat(1000);
    let hash = hash_password(&password);

    assert_eq!(
        hash.len(),
        64,
        "Hash of long password should still be 64 characters"
    );
}

#[test]
fn test_hash_password_whitespace() {
    let password1 = "password";
    let password2 = " password";
    let password3 = "password ";

    let hash1 = hash_password(password1);
    let hash2 = hash_password(password2);
    let hash3 = hash_password(password3);

    assert_ne!(hash1, hash2, "Leading whitespace should change hash");
    assert_ne!(hash1, hash3, "Trailing whitespace should change hash");
    assert_ne!(
        hash2, hash3,
        "Different whitespace positions should produce different hashes"
    );
}
