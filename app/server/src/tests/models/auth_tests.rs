#[cfg(test)]
mod tests {
    use crate::api::auth::{
        ConfirmEmailPayload, EmailConfirmationToken, JwtClaims, LoginPayload, RegisterPayload,
        ResetPasswordPayload, ResetPasswordRequestPayload,
    };
    use crate::api::users::{User, UserRole};

    // ===== JwtClaims tests =====

    #[test]
    fn test_jwt_claims_roundtrip() {
        let user = User::new(
            "testuser".into(),
            "John".into(),
            "Doe".into(),
            "john@test.com".into(),
            "pentester".into(),
            "password",
        );
        let claims = JwtClaims::from(user);
        let secret = "test_secret";

        let token = claims.to_string(secret).unwrap();
        assert!(!token.is_empty());

        let decoded = JwtClaims::from_jwt(&token, secret);
        assert_eq!(decoded.email, "john@test.com");
        assert_eq!(decoded.role, UserRole::Pentester);
    }

    #[test]
    fn test_jwt_claims_invalid_token() {
        let claims = JwtClaims::from_jwt("invalid.token.here", "secret");
        assert_eq!(claims.sub, ulid::Ulid::nil());
        assert!(claims.email.is_empty());
        assert_eq!(claims.role, UserRole::Client);
        assert_eq!(claims.iat, 0);
        assert_eq!(claims.exp, 0);
    }

    #[test]
    fn test_jwt_claims_wrong_secret() {
        let user = User::new(
            "user".into(),
            "A".into(),
            "B".into(),
            "a@b.com".into(),
            "admin".into(),
            "pw",
        );
        let claims = JwtClaims::from(user);
        let token = claims.to_string("secret1").unwrap();

        let decoded = JwtClaims::from_jwt(&token, "wrong_secret");
        assert_eq!(decoded.sub, ulid::Ulid::nil());
    }

    #[test]
    fn test_jwt_claims_is_admin() {
        let user = User::new(
            "u".into(), "A".into(), "B".into(), "a@b.com".into(),
            "admin".into(), "pw",
        );
        let claims = JwtClaims::from(user);
        assert!(claims.is_admin());
        assert!(claims.is_pentester()); // admin is also pentester
    }

    #[test]
    fn test_jwt_claims_is_pentester() {
        let user = User::new(
            "u".into(), "A".into(), "B".into(), "a@b.com".into(),
            "pentester".into(), "pw",
        );
        let claims = JwtClaims::from(user);
        assert!(!claims.is_admin());
        assert!(claims.is_pentester());
    }

    #[test]
    fn test_jwt_claims_client_role() {
        let user = User::new(
            "u".into(), "A".into(), "B".into(), "a@b.com".into(),
            "client".into(), "pw",
        );
        let claims = JwtClaims::from(user);
        assert!(!claims.is_admin());
        assert!(!claims.is_pentester());
    }

    #[test]
    fn test_jwt_claims_is_valid() {
        let user = User::new(
            "u".into(), "A".into(), "B".into(), "a@b.com".into(),
            "pentester".into(), "pw",
        );
        let claims = JwtClaims::from(user);
        assert!(claims.is_valid());
    }

    #[test]
    fn test_jwt_claims_expired_is_invalid() {
        let claims = JwtClaims {
            sub: ulid::Ulid::new(),
            email: "test@test.com".into(),
            role: UserRole::Pentester,
            iat: 1_000_000,
            exp: 1_000_001, // expired long ago
        };
        assert!(!claims.is_valid());
    }

    // ===== EmailConfirmationToken tests =====

    #[test]
    fn test_email_confirmation_token_new() {
        let token = EmailConfirmationToken::new(
            "user".into(),
            "John".into(),
            "Doe".into(),
            "john@test.com".into(),
            "hash123".into(),
        );
        assert_eq!(token.username, "user");
        assert_eq!(token.email, "john@test.com");
        assert!(!token.is_expired());
    }

    #[test]
    fn test_email_confirmation_token_encrypt_decrypt_roundtrip() {
        let key = b"my_super_secret_key_for_testing!";
        let token = EmailConfirmationToken::new(
            "user".into(),
            "John".into(),
            "Doe".into(),
            "john@test.com".into(),
            "hash123".into(),
        );

        let encrypted = token.encrypt(key).unwrap();
        assert!(!encrypted.is_empty());

        let decrypted = EmailConfirmationToken::decrypt(&encrypted, key).unwrap();
        assert_eq!(decrypted.username, "user");
        assert_eq!(decrypted.first_name, "John");
        assert_eq!(decrypted.last_name, "Doe");
        assert_eq!(decrypted.email, "john@test.com");
        assert_eq!(decrypted.password_hash, "hash123");
    }

    #[test]
    fn test_email_confirmation_token_decrypt_wrong_key() {
        let key = b"correct_key_for_encryption_test!";
        let token = EmailConfirmationToken::new(
            "user".into(), "A".into(), "B".into(), "a@b.com".into(), "hash".into(),
        );
        let encrypted = token.encrypt(key).unwrap();

        let result = EmailConfirmationToken::decrypt(&encrypted, b"wrong_key_for_decryption_test!");
        assert!(result.is_err());
    }

    #[test]
    fn test_email_confirmation_token_decrypt_invalid_base64() {
        let result = EmailConfirmationToken::decrypt("not_valid_base64!!!", b"key");
        assert!(result.is_err());
    }

    #[test]
    fn test_email_confirmation_token_decrypt_too_short() {
        use base64::Engine;
        let short = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b"short");
        let result = EmailConfirmationToken::decrypt(&short, b"key_that_is_32_bytes_long_here!");
        assert!(result.is_err());
    }

    // ===== PasswordResetToken tests =====

    #[test]
    fn test_password_reset_token_not_expired() {
        use crate::api::auth::PasswordResetToken;
        let token = PasswordResetToken {
            user_id: "u1".into(),
            token_hash: "hash".into(),
            created_at: chrono::Utc::now().naive_utc(),
        };
        assert!(!token.is_expired());
        assert!(token.is_valid());
    }

    #[test]
    fn test_password_reset_token_expired() {
        use crate::api::auth::PasswordResetToken;
        let old = chrono::Utc::now().naive_utc() - chrono::Duration::minutes(20);
        let token = PasswordResetToken {
            user_id: "u1".into(),
            token_hash: "hash".into(),
            created_at: old,
        };
        assert!(token.is_expired());
        assert!(!token.is_valid());
    }

    // ===== ConfirmationTokenError Display =====

    #[test]
    fn test_confirmation_token_error_display() {
        use crate::api::auth::ConfirmationTokenError;
        let err = ConfirmationTokenError::TokenExpired;
        assert_eq!(err.to_string(), "Token has expired");
    }

    // ===== Payload serde tests =====

    #[test]
    fn test_login_payload_serde() {
        let json = r#"{"email":"a@b.com","password":"pw"}"#;
        let payload: LoginPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.email, "a@b.com");
        assert_eq!(payload.password, "pw");
    }

    #[test]
    fn test_register_payload_serde() {
        let json = r#"{"username":"u","firstName":"A","lastName":"B","email":"a@b.com","password":"pw"}"#;
        let payload: RegisterPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.username, "u");
        assert_eq!(payload.first_name, "A");
        assert_eq!(payload.last_name, "B");
    }

    #[test]
    fn test_reset_password_request_payload_serde() {
        let json = r#"{"resetEmail":"a@b.com"}"#;
        let payload: ResetPasswordRequestPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.reset_email, "a@b.com");
    }

    #[test]
    fn test_reset_password_payload_serde() {
        let json = r#"{"token":"tok123","newPassword":"newpw"}"#;
        let payload: ResetPasswordPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.token, "tok123");
        assert_eq!(payload.new_password, "newpw");
    }

    #[test]
    fn test_confirm_email_payload_serde() {
        let json = r#"{"token":"tok123"}"#;
        let payload: ConfirmEmailPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.token, "tok123");
    }

    // ===== User from RegisterPayload =====

    #[test]
    fn test_user_from_register_payload() {
        let payload = RegisterPayload {
            username: "newuser".into(),
            first_name: "New".into(),
            last_name: "User".into(),
            email: "new@test.com".into(),
            password: "strongpw".into(),
        };
        let user = User::from(payload);
        assert_eq!(user.username, "newuser");
        assert_eq!(user.email, "new@test.com");
        assert_eq!(user.role, UserRole::Pentester); // default role
        assert!(!user.id.is_empty());
    }
}
