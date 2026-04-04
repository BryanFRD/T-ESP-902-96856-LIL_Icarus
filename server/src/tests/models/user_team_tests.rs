#[cfg(test)]
mod tests {
    use crate::api::users::{User, UserRole, UserResponse, UpdatePayload, avatar_url_path, map_avatar_url};
    use crate::api::teams::{Team, TeamRole, CreateTeam, UpdateTeam, TeamReturn, UpdateMemberRolePayload};

    // ===== UserRole tests =====

    #[test]
    fn test_user_role_display() {
        assert_eq!(UserRole::Admin.to_string(), "admin");
        assert_eq!(UserRole::Pentester.to_string(), "pentester");
        assert_eq!(UserRole::Client.to_string(), "client");
    }

    #[test]
    fn test_user_role_from_string() {
        assert_eq!(UserRole::from("admin".to_string()), UserRole::Admin);
        assert_eq!(UserRole::from("pentester".to_string()), UserRole::Pentester);
        assert_eq!(UserRole::from("client".to_string()), UserRole::Client);
        assert_eq!(UserRole::from("unknown".to_string()), UserRole::Client); // default
    }

    #[test]
    fn test_user_role_serde() {
        let json = serde_json::to_string(&UserRole::Admin).unwrap();
        assert_eq!(json, "\"ADMIN\"");
        let json = serde_json::to_string(&UserRole::Pentester).unwrap();
        assert_eq!(json, "\"PENTESTER\"");
        let json = serde_json::to_string(&UserRole::Client).unwrap();
        assert_eq!(json, "\"CLIENT\"");

        let deserialized: UserRole = serde_json::from_str("\"ADMIN\"").unwrap();
        assert_eq!(deserialized, UserRole::Admin);
    }

    // ===== User tests =====

    fn make_user() -> User {
        User::new(
            "testuser".into(), "John".into(), "Doe".into(),
            "john@test.com".into(), "pentester".into(), "password123",
        )
    }

    #[test]
    fn test_user_needs_rehash_none() {
        let mut user = make_user();
        user.password_hash_version = None;
        assert!(user.needs_rehash());
    }

    #[test]
    fn test_user_needs_rehash_v1() {
        let mut user = make_user();
        user.password_hash_version = Some(1);
        assert!(user.needs_rehash());
    }

    #[test]
    fn test_user_no_rehash_v2() {
        let user = make_user();
        assert_eq!(user.password_hash_version, Some(2));
        assert!(!user.needs_rehash());
    }

    #[test]
    fn test_user_serialization_hides_password() {
        let user = make_user();
        let json = serde_json::to_value(&user).unwrap();
        assert!(json.get("password_hash").is_none());
        assert!(json.get("passwordHash").is_none());
        assert!(json.get("is_connected").is_none());
        assert!(json.get("isConnected").is_none());
    }

    // ===== UserResponse =====

    #[test]
    fn test_user_response_from_user() {
        let user = make_user();
        let resp = UserResponse::from(user.clone());
        assert_eq!(resp.id, user.id);
        assert_eq!(resp.username, "testuser");
        assert_eq!(resp.email, "john@test.com");
        assert!(resp.avatar_url.is_none());
    }

    #[test]
    fn test_user_response_maps_avatar_url() {
        let mut user = make_user();
        user.avatar_url = Some("abc123.jpg".into());
        let resp = UserResponse::from(user);
        assert_eq!(resp.avatar_url.as_deref(), Some("/v1/users/avatars/abc123.jpg"));
    }

    // ===== avatar helpers =====

    #[test]
    fn test_avatar_url_path() {
        assert_eq!(avatar_url_path("file.jpg"), "/v1/users/avatars/file.jpg");
    }

    #[test]
    fn test_map_avatar_url_some() {
        let result = map_avatar_url(Some("file.jpg".into()));
        assert_eq!(result.as_deref(), Some("/v1/users/avatars/file.jpg"));
    }

    #[test]
    fn test_map_avatar_url_none() {
        let result = map_avatar_url(None);
        assert!(result.is_none());
    }

    // ===== UpdatePayload =====

    #[test]
    fn test_update_payload_serde() {
        let json = r#"{"username":"new_name"}"#;
        let payload: UpdatePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.username.as_deref(), Some("new_name"));
        assert!(payload.email.is_none());
        assert!(payload.password.is_none());
    }

    #[test]
    fn test_update_payload_with_password_change() {
        let json = r#"{"password":"old","newPassword":"new","confirmPassword":"new"}"#;
        let payload: UpdatePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.password.as_deref(), Some("old"));
        assert_eq!(payload.new_password.as_deref(), Some("new"));
        assert_eq!(payload.confirm_password.as_deref(), Some("new"));
    }

    // ===== Team tests =====

    #[test]
    fn test_team_new() {
        let team = Team::new("Alpha".into(), "A team".into());
        assert_eq!(team.name, "Alpha");
        assert_eq!(team.description, "A team");
        assert!(!team.id.is_empty());
    }

    #[test]
    fn test_team_unique_ids() {
        let t1 = Team::new("A".into(), "B".into());
        let t2 = Team::new("A".into(), "B".into());
        assert_ne!(t1.id, t2.id);
    }

    // ===== TeamReturn =====

    #[test]
    fn test_team_return_new() {
        let team = Team::new("Alpha".into(), "Desc".into());
        let tr = TeamReturn::new(team.clone(), vec![], vec![]);
        assert_eq!(tr.id, team.id);
        assert_eq!(tr.name, "Alpha");
        assert!(tr.members.is_empty());
        assert!(tr.companies.is_empty());
    }

    // ===== Payload serde =====

    #[test]
    fn test_create_team_serde() {
        let json = r#"{"name":"Team","description":"Desc"}"#;
        let payload: CreateTeam = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "Team");
        assert_eq!(payload.description, "Desc");
    }

    #[test]
    fn test_update_team_serde() {
        let json = r#"{"name":"New Name"}"#;
        let payload: UpdateTeam = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name.as_deref(), Some("New Name"));
        assert!(payload.description.is_none());
    }

    #[test]
    fn test_update_member_role_payload_serde() {
        let json = r#"{"role":"OWNER"}"#;
        let payload: UpdateMemberRolePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.role, TeamRole::Owner);
    }

    // ===== TeamRole tests =====

    #[test]
    fn test_team_role_serde_roundtrip() {
        for (variant, expected_json) in [
            (TeamRole::Owner, "\"OWNER\""),
            (TeamRole::Moderator, "\"MODERATOR\""),
            (TeamRole::Member, "\"MEMBER\""),
            (TeamRole::Client, "\"CLIENT\""),
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: TeamRole = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }
}
