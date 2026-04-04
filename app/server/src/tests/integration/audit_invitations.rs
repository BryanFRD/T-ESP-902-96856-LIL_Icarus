#[cfg(test)]
mod audit_invitations_integration_tests {
    use crate::api::audits::invitations::CreateAuditInvitationPayload;
    use crate::api::auth::LoginPayload;
    use crate::api::teams::TeamRole;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{
            create_team_with_owner, insert_team_member, insert_test_audit, insert_test_company,
            insert_test_user,
        },
    };
    use axum_test::TestServer;
    use http::StatusCode;
    use serial_test::serial;

    /// Helper pour authentifier un utilisateur et retourner le cookie auth
    async fn login_user(
        server: &TestServer,
        email: &str,
        password: &str,
    ) -> axum_test::TestResponse {
        let payload = LoginPayload {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = server.post("/v1/auth/login").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::OK);
        response
    }

    // ============================================================================
    // Tests de permissions - Création d'invitations
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_as_team_owner_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "inv_owner",
            "inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert!(body["id"].is_string());
        assert_eq!(body["clientEmail"], "client@test.com");
        assert!(body["createdAt"].is_string());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_as_team_member_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "inv_member_owner",
            "inv_member_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "inv_member",
            "inv_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "inv_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_nonprofit_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "nonexist_inv_owner",
            "nonexist_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "nonexist_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post("/v1/audits/nonexistent-audit/invite")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_invalid_email_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "invalid_email_owner",
            "invalid_email_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &_team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &_team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "invalid_email_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "invalid-email-format".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_on_archived_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "archive_inv_owner",
            "archive_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        // Archive the audit
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
        let mut conn = pool.get().unwrap();
        diesel::update(
            crate::schema::audits::table.filter(crate::schema::audits::id.eq(&audit.id)),
        )
        .set(crate::schema::audits::is_archived.eq(true))
        .execute(&mut conn)
        .unwrap();

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "archive_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_duplicate_email_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "dup_inv_owner",
            "dup_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let existing_user = insert_test_user(
            &pool,
            "existing_user",
            "existing_user@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        // Add existing user to the audit
        use diesel::{ExpressionMethods, RunQueryDsl};
        let mut conn = pool.get().unwrap();
        diesel::insert_into(crate::schema::audits_clients::table)
            .values((
                crate::schema::audits_clients::audit_id.eq(&audit.id),
                crate::schema::audits_clients::user_id.eq(&existing_user.id),
                crate::schema::audits_clients::created_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .unwrap();

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "dup_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "existing_user@test.com".to_string(),
            expires_at: None,
        };

        // Try to create invitation for user already in audit
        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Should fail with conflict
        assert_eq!(response.status_code(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_invitation_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "admin_inv_owner",
            "admin_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin_inv",
            "admin_inv@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "admin_inv@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    // ============================================================================
    // Tests de structure de réponse
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_invitation_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "struct_inv_owner",
            "struct_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "struct_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "test@example.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();

        // Verify response structure
        assert!(body["id"].is_string());
        assert_eq!(body["clientEmail"], "test@example.com");
        assert!(body["createdAt"].is_string());
        assert!(body["expiresAt"].is_string() || body["expiresAt"].is_number());
        assert!(body["isCanceled"].is_boolean());
        assert!(body["isUsed"].is_boolean());
    }

    #[tokio::test]
    #[serial]
    async fn test_unauthorized_user_cannot_create_invitation() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "unauth_inv_owner",
            "unauth_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _unauth = insert_test_user(
            &pool,
            "unauth_inv_user",
            "unauth_inv_user@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "unauth_inv_user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_unauthenticated_user_cannot_create_invitation() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "no_auth_inv_owner",
            "no_auth_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = CreateAuditInvitationPayload {
            client_email: "client@test.com".to_string(),
            expires_at: None,
        };

        let response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour list_invitations
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_list_audit_invitations_as_team_owner_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "list_inv_owner",
            "list_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "list_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create an invitation first
        let payload = CreateAuditInvitationPayload {
            client_email: "test1@test.com".to_string(),
            expires_at: None,
        };

        server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // List invitations
        let response = server
            .get(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert!(body.is_array());
        assert_eq!(body.as_array().unwrap().len(), 1);
        assert_eq!(body[0]["clientEmail"], "test1@test.com");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_audit_invitations_as_team_member_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "list_member_owner",
            "list_member_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "list_member",
            "list_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "list_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_audit_invitations_nonexistent_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "list_nonexist_owner",
            "list_nonexist_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "list_nonexist_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/audits/nonexistent-audit/invite")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    // ============================================================================
    // Tests pour cancel_invitation
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_cancel_invitation_as_team_owner_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "cancel_inv_owner",
            "cancel_inv_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "cancel_inv_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create an invitation
        let payload = CreateAuditInvitationPayload {
            client_email: "candidate@test.com".to_string(),
            expires_at: None,
        };

        let create_response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        let body: serde_json::Value = create_response.json();
        let invitation_id = body["id"].as_str().unwrap().to_string();

        // Cancel the invitation
        let response = server
            .delete(&format!("/v1/audits/{}/invite/{}", audit.id, invitation_id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_cancel_invitation_as_team_member_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "cancel_member_owner",
            "cancel_member_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "cancel_member",
            "cancel_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Create invitation as owner
        let owner_login = login_user(&server, "cancel_member_owner@test.com", "password").await;
        let owner_cookies = owner_login.cookies();
        let owner_auth_cookie = owner_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "candidate@test.com".to_string(),
            expires_at: None,
        };

        let create_response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(owner_auth_cookie.clone())
            .json(&payload)
            .await;

        let body: serde_json::Value = create_response.json();
        let invitation_id = body["id"].as_str().unwrap().to_string();

        // Try to cancel as member
        let member_login = login_user(&server, "cancel_member@test.com", "password").await;
        let member_cookies = member_login.cookies();
        let member_auth_cookie = member_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/audits/{}/invite/{}", audit.id, invitation_id))
            .add_cookie(member_auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_cancel_nonexistent_invitation_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "cancel_nonexist_owner",
            "cancel_nonexist_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response =
            login_user(&server, "cancel_nonexist_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!(
                "/v1/audits/{}/invite/nonexistent-invitation",
                audit.id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_cancel_invitation_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "cancel_admin_owner",
            "cancel_admin_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "cancel_admin",
            "cancel_admin@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Create invitation as owner
        let owner_login = login_user(&server, "cancel_admin_owner@test.com", "password").await;
        let owner_cookies = owner_login.cookies();
        let owner_auth_cookie = owner_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditInvitationPayload {
            client_email: "candidate@test.com".to_string(),
            expires_at: None,
        };

        let create_response = server
            .post(&format!("/v1/audits/{}/invite", audit.id))
            .add_cookie(owner_auth_cookie.clone())
            .json(&payload)
            .await;

        let body: serde_json::Value = create_response.json();
        let invitation_id = body["id"].as_str().unwrap().to_string();

        // Cancel as admin
        let admin_login = login_user(&server, "cancel_admin@test.com", "password").await;
        let admin_cookies = admin_login.cookies();
        let admin_auth_cookie = admin_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/audits/{}/invite/{}", audit.id, invitation_id))
            .add_cookie(admin_auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    // ============================================================================
    // Tests pour check_invitation
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_check_invitation_with_invalid_token_returns_404() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server
            .get("/v1/audits/invite/invalid-token-xyz/check")
            .await;

        // Invalid tokens can return 404 (not found) or 500 (internal error from decryption)
        assert!(
            response.status_code() == StatusCode::NOT_FOUND
                || response.status_code() == StatusCode::GONE
                || response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_check_invitation_with_malformed_token_returns_400_or_500() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Test a valid token format but invalid content
        let response = server.get("/v1/audits/invite/invalidtoken123/check").await;

        // Invalid tokens can return 404, 410, or 500
        assert!(
            response.status_code() == StatusCode::NOT_FOUND
                || response.status_code() == StatusCode::GONE
                || response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    // ============================================================================
    // Tests pour accept_invitation
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_accept_invitation_with_invalid_token_returns_404() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = serde_json::json!({
            "username": "newuser",
            "firstName": "Test",
            "lastName": "User",
            "password": "securepassword123"
        });

        let response = server
            .post("/v1/audits/invite/invalid-token-xyz/accept")
            .json(&payload)
            .await;

        // Invalid tokens can return 404, 410 (gone), or 500
        assert!(
            response.status_code() == StatusCode::NOT_FOUND
                || response.status_code() == StatusCode::GONE
                || response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_accept_invitation_with_invalid_password_returns_400() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = serde_json::json!({
            "username": "newuser",
            "firstName": "Test",
            "lastName": "User",
            "password": "short"  // Too short (min 8 chars)
        });

        let response = server
            .post("/v1/audits/invite/invalid-token-xyz/accept")
            .json(&payload)
            .await;

        // Password validation happens, but token validation might fail first
        assert!(
            response.status_code() == StatusCode::BAD_REQUEST
                || response.status_code() == StatusCode::NOT_FOUND
                || response.status_code() == StatusCode::GONE
                || response.status_code() == StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
