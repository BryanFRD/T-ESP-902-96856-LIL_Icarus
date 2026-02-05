use crate::api::auth::RegisterPayload;
use crate::api::users::{User, UserRole};

// Tests for User::new()
#[test]
fn test_user_new_creates_valid_user() {
    let user = User::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "admin".to_string(),
        "password123",
    );

    assert_eq!(user.username, "testuser");
    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.role, UserRole::Admin);
    assert!(!user.id.is_empty(), "ID should be generated");
    assert_eq!(user.is_connected, false);
    assert_eq!(user.avatar_url, None);
}

#[test]
fn test_user_new_hashes_password() {
    let password = "test_password_123";
    let user = User::new(
        "testuser".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        "john@example.com".to_string(),
        "pentester".to_string(),
        password,
    );

    // User::new() uses Argon2id hashing (version 2), which generates random salts
    // so we can't compare hashes directly. Instead, verify the password works.
    assert!(
        user.verify_password(password),
        "Password should be correctly hashed and verifiable"
    );
    assert_ne!(
        user.password_hash, password,
        "Password should be hashed, not stored in plain text"
    );
    assert!(
        user.password_hash.starts_with("$argon2id$"),
        "Password hash should use Argon2id format"
    );
    assert_eq!(
        user.password_hash_version,
        Some(2),
        "Password hash version should be 2 for Argon2id"
    );
}

#[test]
fn test_user_new_sets_role_correctly() {
    let admin_user = User::new(
        "admin".to_string(),
        "Admin".to_string(),
        "User".to_string(),
        "admin@example.com".to_string(),
        "admin".to_string(),
        "password",
    );
    assert_eq!(admin_user.role, UserRole::Admin);

    let pentester_user = User::new(
        "pentester".to_string(),
        "Pentester".to_string(),
        "User".to_string(),
        "pentester@example.com".to_string(),
        "pentester".to_string(),
        "password",
    );
    assert_eq!(pentester_user.role, UserRole::Pentester);

    let client_user = User::new(
        "client".to_string(),
        "Client".to_string(),
        "User".to_string(),
        "client@example.com".to_string(),
        "client".to_string(),
        "password",
    );
    assert_eq!(client_user.role, UserRole::Client);
}

#[test]
fn test_user_new_generates_unique_ids() {
    let user1 = User::new(
        "user1".to_string(),
        "User".to_string(),
        "One".to_string(),
        "user1@example.com".to_string(),
        "client".to_string(),
        "password",
    );

    let user2 = User::new(
        "user2".to_string(),
        "User".to_string(),
        "Two".to_string(),
        "user2@example.com".to_string(),
        "client".to_string(),
        "password",
    );

    assert_ne!(user1.id, user2.id, "Each user should have a unique ID");
}

// Tests for User::verify_password()
#[test]
fn test_user_verify_password_correct() {
    let password = "correct_password";
    let user = User::new(
        "testuser".to_string(),
        "Test".to_string(),
        "User".to_string(),
        "test@example.com".to_string(),
        "client".to_string(),
        password,
    );

    assert!(
        user.verify_password(password),
        "Correct password should verify successfully"
    );
}

#[test]
fn test_user_verify_password_incorrect() {
    let user = User::new(
        "testuser".to_string(),
        "Test".to_string(),
        "User".to_string(),
        "test@example.com".to_string(),
        "client".to_string(),
        "correct_password",
    );

    assert!(
        !user.verify_password("wrong_password"),
        "Wrong password should fail verification"
    );
}

#[test]
fn test_user_verify_password_case_sensitive() {
    let user = User::new(
        "testuser".to_string(),
        "Test".to_string(),
        "User".to_string(),
        "test@example.com".to_string(),
        "client".to_string(),
        "Password123",
    );

    assert!(
        !user.verify_password("password123"),
        "Password verification should be case-sensitive"
    );
    assert!(
        !user.verify_password("PASSWORD123"),
        "Password verification should be case-sensitive"
    );
}

#[test]
fn test_user_verify_password_empty_string() {
    let user = User::new(
        "testuser".to_string(),
        "Test".to_string(),
        "User".to_string(),
        "test@example.com".to_string(),
        "client".to_string(),
        "nonempty",
    );

    assert!(
        !user.verify_password(""),
        "Empty password should not match non-empty hash"
    );
}

// Tests for From<RegisterPayload> for User
#[test]
fn test_user_from_register_payload() {
    let payload = RegisterPayload {
        username: "newuser".to_string(),
        first_name: "New".to_string(),
        last_name: "User".to_string(),
        email: "newuser@example.com".to_string(),
        password: "securepassword".to_string(),
    };

    let user = User::from(payload.clone());

    assert_eq!(user.username, payload.username);
    assert_eq!(user.first_name, payload.first_name);
    assert_eq!(user.last_name, payload.last_name);
    assert_eq!(user.email, payload.email);
    assert_eq!(
        user.role,
        UserRole::Pentester,
        "Default role should be Pentester"
    );
    assert!(
        user.verify_password(&payload.password),
        "Password should be correctly hashed"
    );
}

#[test]
fn test_user_from_register_payload_always_pentester() {
    let payload = RegisterPayload {
        username: "user".to_string(),
        first_name: "First".to_string(),
        last_name: "Last".to_string(),
        email: "user@example.com".to_string(),
        password: "password".to_string(),
    };

    let user = User::from(payload);

    assert_eq!(
        user.role,
        UserRole::Pentester,
        "RegisterPayload conversion should always create Pentester role"
    );
}

// Tests for UserRole Display
#[test]
fn test_user_role_display() {
    assert_eq!(format!("{}", UserRole::Admin), "admin");
    assert_eq!(format!("{}", UserRole::Pentester), "pentester");
    assert_eq!(format!("{}", UserRole::Client), "client");
}

// Tests for UserRole From<String>
#[test]
fn test_user_role_from_string_admin() {
    let role = UserRole::from("admin".to_string());
    assert_eq!(role, UserRole::Admin);
}

#[test]
fn test_user_role_from_string_pentester() {
    let role = UserRole::from("pentester".to_string());
    assert_eq!(role, UserRole::Pentester);
}

#[test]
fn test_user_role_from_string_client() {
    let role = UserRole::from("client".to_string());
    assert_eq!(role, UserRole::Client);
}

#[test]
fn test_user_role_from_string_default_to_client() {
    let role1 = UserRole::from("unknown".to_string());
    assert_eq!(
        role1,
        UserRole::Client,
        "Unknown role should default to Client"
    );

    let role2 = UserRole::from("".to_string());
    assert_eq!(
        role2,
        UserRole::Client,
        "Empty string should default to Client"
    );

    let role3 = UserRole::from("ADMIN".to_string());
    assert_eq!(
        role3,
        UserRole::Client,
        "Case-sensitive: uppercase should default to Client"
    );
}

#[test]
fn test_user_role_from_string_case_sensitive() {
    // Test that conversion is case-sensitive
    assert_eq!(UserRole::from("Admin".to_string()), UserRole::Client);
    assert_eq!(UserRole::from("PENTESTER".to_string()), UserRole::Client);
    assert_eq!(UserRole::from("Client".to_string()), UserRole::Client);
}
