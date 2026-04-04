#[cfg(test)]
mod tests {
    use crate::api::auth::{hash_password_argon2id, verify_password_with_migration};

    // hash_password_sha256 is pub but only in services - access via re-export
    fn sha256_hash(password: &str) -> String {
        crate::api::auth::hash_password(password)
    }

    // ===== SHA-256 legacy hashing =====

    #[test]
    fn test_sha256_hash_deterministic() {
        let h1 = sha256_hash("password");
        let h2 = sha256_hash("password");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_sha256_hash_different_inputs() {
        let h1 = sha256_hash("password1");
        let h2 = sha256_hash("password2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_sha256_hash_not_empty() {
        let h = sha256_hash("test");
        assert!(!h.is_empty());
        // SHA-256 output is 64 hex chars
        assert_eq!(h.len(), 64);
    }

    // ===== Argon2id hashing =====

    #[test]
    fn test_argon2id_hash_format() {
        let hash = hash_password_argon2id("password");
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn test_argon2id_hash_unique_salts() {
        let h1 = hash_password_argon2id("password");
        let h2 = hash_password_argon2id("password");
        assert_ne!(h1, h2, "Different salts should produce different hashes");
    }

    // ===== verify_password_with_migration =====

    #[test]
    fn test_verify_argon2id_correct() {
        let hash = hash_password_argon2id("mypassword");
        assert!(verify_password_with_migration("mypassword", &hash, Some(2)));
    }

    #[test]
    fn test_verify_argon2id_incorrect() {
        let hash = hash_password_argon2id("mypassword");
        assert!(!verify_password_with_migration("wrong", &hash, Some(2)));
    }

    #[test]
    fn test_verify_argon2id_invalid_hash() {
        assert!(!verify_password_with_migration("pw", "not_a_valid_hash", Some(2)));
    }

    #[test]
    fn test_verify_sha256_correct() {
        let hash = sha256_hash("mypassword");
        assert!(verify_password_with_migration("mypassword", &hash, Some(1)));
    }

    #[test]
    fn test_verify_sha256_correct_none_version() {
        let hash = sha256_hash("mypassword");
        assert!(verify_password_with_migration("mypassword", &hash, None));
    }

    #[test]
    fn test_verify_sha256_incorrect() {
        let hash = sha256_hash("mypassword");
        assert!(!verify_password_with_migration("wrong", &hash, Some(1)));
    }

    #[test]
    fn test_verify_unknown_version() {
        assert!(!verify_password_with_migration("pw", "hash", Some(99)));
    }
}
