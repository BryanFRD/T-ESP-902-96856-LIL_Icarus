use crate::api::{auth::JwtClaims, users::{User, UserRole}};
use ulid::Ulid;
use chrono::Utc;

#[test]
fn test_jwt_claims_encode_decode() {
    let secret = "test_secret_key_for_jwt";
    let claims = JwtClaims {
        sub: Ulid::new(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        iat: usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0),
        exp: usize::try_from((chrono::Local::now() + chrono::Duration::days(1)).timestamp())
            .unwrap_or(0),
    };

    let encoded = claims.to_string(secret).expect("Should encode JWT");
    let decoded = JwtClaims::from_jwt(&encoded, secret);

    assert_eq!(decoded.sub, claims.sub, "Subject should match");
    assert_eq!(decoded.email, claims.email, "Email should match");
    assert_eq!(decoded.role, claims.role, "Role should match");
}

#[test]
fn test_jwt_claims_invalid_secret() {
    let secret = "correct_secret";
    let wrong_secret = "wrong_secret";

    let claims = JwtClaims {
        sub: Ulid::new(),
        email: "test@example.com".to_string(),
        role: UserRole::Admin,
        iat: usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0),
        exp: usize::try_from((chrono::Local::now() + chrono::Duration::days(1)).timestamp())
            .unwrap_or(0),
    };

    let encoded = claims.to_string(secret).expect("Should encode JWT");
    let decoded = JwtClaims::from_jwt(&encoded, wrong_secret);

    assert_eq!(decoded.sub, Ulid::nil(), "Invalid JWT should return nil Ulid");
    assert_eq!(decoded.email, "", "Invalid JWT should return empty email");
    assert_eq!(decoded.role, UserRole::Client, "Invalid JWT should return Client role");
}

#[test]
fn test_jwt_claims_is_admin() {
    let admin_claims = JwtClaims {
        sub: Ulid::new(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        iat: 0,
        exp: 0,
    };

    let pentester_claims = JwtClaims {
        sub: Ulid::new(),
        email: "pentester@example.com".to_string(),
        role: UserRole::Pentester,
        iat: 0,
        exp: 0,
    };

    let client_claims = JwtClaims {
        sub: Ulid::new(),
        email: "client@example.com".to_string(),
        role: UserRole::Client,
        iat: 0,
        exp: 0,
    };

    assert!(admin_claims.is_admin(), "Admin role should return true");
    assert!(!pentester_claims.is_admin(), "Pentester role should return false");
    assert!(!client_claims.is_admin(), "Client role should return false");
}

#[test]
fn test_jwt_claims_is_pentester() {
    let admin_claims = JwtClaims {
        sub: Ulid::new(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        iat: 0,
        exp: 0,
    };

    let pentester_claims = JwtClaims {
        sub: Ulid::new(),
        email: "pentester@example.com".to_string(),
        role: UserRole::Pentester,
        iat: 0,
        exp: 0,
    };

    let client_claims = JwtClaims {
        sub: Ulid::new(),
        email: "client@example.com".to_string(),
        role: UserRole::Client,
        iat: 0,
        exp: 0,
    };

    assert!(admin_claims.is_pentester(), "Admin should be considered pentester");
    assert!(pentester_claims.is_pentester(), "Pentester should return true");
    assert!(!client_claims.is_pentester(), "Client should return false");
}

#[test]
fn test_jwt_claims_is_valid_not_expired() {
    let now = usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0);
    let claims = JwtClaims {
        sub: Ulid::new(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        iat: now.saturating_sub(3600),
        exp: now + 3600,
    };

    assert!(claims.is_valid(), "Token with valid iat and exp should be valid");
}

#[test]
fn test_jwt_claims_is_valid_expired() {
    let now = usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0);
    let claims = JwtClaims {
        sub: Ulid::new(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        iat: now.saturating_sub(7200),
        exp: now.saturating_sub(3600),
    };

    assert!(!claims.is_valid(), "Expired token should not be valid");
}

#[test]
fn test_jwt_claims_is_valid_not_yet_valid() {
    let now = usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0);
    let claims = JwtClaims {
        sub: Ulid::new(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        iat: now + 3600,
        exp: now + 7200,
    };

    assert!(!claims.is_valid(), "Token not yet valid (iat in future) should not be valid");
}

#[test]
fn test_jwt_claims_malformed_token() {
    let secret = "test_secret";
    let malformed = "this.is.not.a.valid.jwt.token";

    let decoded = JwtClaims::from_jwt(malformed, secret);

    assert_eq!(decoded.sub, Ulid::nil(), "Malformed JWT should return nil Ulid");
    assert_eq!(decoded.email, "", "Malformed JWT should return empty email");
    assert_eq!(decoded.role, UserRole::Client, "Malformed JWT should return Client role");
    assert_eq!(decoded.iat, 0, "Malformed JWT should return 0 iat");
    assert_eq!(decoded.exp, 0, "Malformed JWT should return 0 exp");
}

#[test]
fn test_jwt_claims_empty_token() {
    let secret = "test_secret";
    let decoded = JwtClaims::from_jwt("", secret);

    assert_eq!(decoded.sub, Ulid::nil(), "Empty JWT should return nil Ulid");
    assert_eq!(decoded.email, "", "Empty JWT should return empty email");
}

#[test]
fn test_jwt_claims_from_user_valid_ulid() {
    let user_id = Ulid::new();
    let user = User {
        id: user_id.to_string(),
        username: "testuser".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        password_hash: "hashed".to_string(),
        password_hash_version: None,
        is_connected: false,
        avatar_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        last_connected_at: Utc::now().naive_utc(),
    };

    let claims = JwtClaims::from(user.clone());

    assert_eq!(claims.sub, user_id, "Subject should match user ID");
    assert_eq!(claims.email, user.email, "Email should match");
    assert_eq!(claims.role, user.role, "Role should match");
    assert!(claims.iat > 0, "iat should be set");
    assert!(claims.exp > claims.iat, "exp should be after iat");
}

#[test]
fn test_jwt_claims_from_user_invalid_ulid() {
    let user = User {
        id: "not-a-valid-ulid".to_string(),
        username: "testuser".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Client,
        password_hash: "hashed".to_string(),
        password_hash_version: None,
        is_connected: false,
        avatar_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        last_connected_at: Utc::now().naive_utc(),
    };

    let claims = JwtClaims::from(user);

    assert_eq!(claims.sub, Ulid::nil(), "Invalid ULID should default to nil");
}

#[test]
fn test_jwt_claims_from_user_admin_role() {
    let user = User {
        id: Ulid::new().to_string(),
        username: "admin".to_string(),
        first_name: "Admin".to_string(),
        last_name: "User".to_string(),
        email: "admin@example.com".to_string(),
        role: UserRole::Admin,
        password_hash: "hashed".to_string(),
        password_hash_version: None,
        is_connected: true,
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        last_connected_at: Utc::now().naive_utc(),
    };

    let claims = JwtClaims::from(user);

    assert_eq!(claims.role, UserRole::Admin, "Admin role should be preserved");
    assert!(claims.is_admin(), "Should be recognized as admin");
    assert!(claims.is_pentester(), "Admin should also be pentester");
}

#[test]
fn test_jwt_claims_from_user_expiry_one_day() {
    let user = User {
        id: Ulid::new().to_string(),
        username: "testuser".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::Pentester,
        password_hash: "hashed".to_string(),
        password_hash_version: None,
        is_connected: false,
        avatar_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        last_connected_at: Utc::now().naive_utc(),
    };

    let claims = JwtClaims::from(user);

    let now = usize::try_from(chrono::Local::now().timestamp()).unwrap_or(0);
    let one_day_from_now = usize::try_from((chrono::Local::now() + chrono::Duration::days(1)).timestamp()).unwrap_or(0);

    assert!(claims.exp >= one_day_from_now - 2 && claims.exp <= one_day_from_now + 2,
        "Expiry should be approximately 1 day from now");
    assert!(claims.iat >= now - 2 && claims.iat <= now + 2,
        "iat should be approximately now");
}
