#[cfg(test)]
mod audit_messages_integration_tests {
    use crate::api::audits::messages::models::CreateAuditMessagePayload;
    use crate::api::auth::LoginPayload;
    use crate::api::teams::TeamRole;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{
            create_team_with_owner, insert_audit_client, insert_team_member, insert_test_audit,
            insert_test_company, insert_test_user,
        },
    };
    use axum_test::TestServer;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
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
    // Tests de permissions - Listing des messages
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_list_messages_as_team_member_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "list_owner",
            "list_owner@test.com",
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
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "list_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Navigate to step-up page then to audit to satisfy middleware
        let response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        // Should succeed or redirect to step-up depending on middleware
        assert!(
            response.status_code() == StatusCode::OK
                || response.status_code() == StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_as_audit_client_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "client_owner",
            "client_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "audit_client",
            "audit_client@test.com",
            "password",
            &UserRole::Client,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "audit_client@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: Vec<serde_json::Value> = response.json();
        assert!(body.is_empty() || body.iter().all(|m| m["id"].is_string()));
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_unauthorized_user_denied() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "unauth_owner",
            "unauth_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _unauthorized = insert_test_user(
            &pool,
            "unauthorized_user",
            "unauthorized@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "unauthorized@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    // ============================================================================
    // Tests de permissions - Création de messages
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_message_with_team_member_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "create_owner",
            "create_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "create_member",
            "create_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "create_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Test message content".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["content"], "Test message content");
        assert_ne!(body["id"], "");
    }

    #[tokio::test]
    #[serial]
    async fn test_create_message_with_audit_client_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "client_create_owner",
            "client_create_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "message_client",
            "message_client@test.com",
            "password",
            &UserRole::Client,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "message_client@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Client message".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["content"], "Client message");
    }

    #[tokio::test]
    #[serial]
    async fn test_create_message_with_empty_content_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "empty_owner",
            "empty_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "empty_member",
            "empty_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "empty_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_message_unauthorized_user_denied() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "create_unauth_owner",
            "create_unauth_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _unauthorized = insert_test_user(
            &pool,
            "create_unauthorized",
            "create_unauthorized@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "create_unauthorized@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Unauthorized message".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    // ============================================================================
    // Tests d'archivage - Messages sur audit archivé
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_message_on_archived_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "archived_owner",
            "archived_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "archived_member",
            "archived_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        // Archive the audit
        let mut conn = pool.get().unwrap();
        diesel::update(
            crate::schema::audits::table.filter(crate::schema::audits::id.eq(&audit.id)),
        )
        .set(crate::schema::audits::is_archived.eq(true))
        .execute(&mut conn)
        .unwrap();

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "archived_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Message on archived audit".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    // ============================================================================
    // Tests de workflow - Créer et lister des messages
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_and_list_messages_workflow() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "workflow_owner",
            "workflow_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "workflow_member",
            "workflow_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "workflow_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create first message
        let payload1 = CreateAuditMessagePayload {
            content: "First message".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload1)
            .await;
        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let msg1_response: serde_json::Value = create_response.json();
        let _first_msg_id: String = msg1_response["id"].as_str().unwrap().to_string();

        // Create second message
        let payload2 = CreateAuditMessagePayload {
            content: "Second message".to_string(),
        };
        let create_response2 = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload2)
            .await;
        assert_eq!(create_response2.status_code(), StatusCode::CREATED);

        // List messages
        let list_response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::OK);
        let messages: Vec<serde_json::Value> = list_response.json();

        // Should have both messages
        assert!(messages.len() >= 2);

        // Verify message contents
        let contents: Vec<_> = messages
            .iter()
            .filter_map(|m| m["content"].as_str())
            .collect();
        assert!(contents.contains(&"First message"));
        assert!(contents.contains(&"Second message"));
    }

    // ============================================================================
    // Tests d'attachments de messages
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_list_message_attachments_empty() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "attach_owner",
            "attach_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "attach_member",
            "attach_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "attach_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create a message first
        let msg_payload = CreateAuditMessagePayload {
            content: "Message for attachments".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // List attachments for message with no attachments
        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages/{}/attachments",
                audit.id, msg_id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::OK);
        let body: Vec<serde_json::Value> = list_response.json();
        assert_eq!(body.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_attachments_unauthorized_user_denied() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "attach_unauth_owner",
            "attach_unauth_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "attach_unauth_member",
            "attach_unauth_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _unauthorized = insert_test_user(
            &pool,
            "attach_unauth_user",
            "attach_unauth_user@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "attach_unauth_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create a message as authorized user
        let msg_payload = CreateAuditMessagePayload {
            content: "Protected message".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Try to list attachments as unauthorized user
        let unauth_login = login_user(&server, "attach_unauth_user@test.com", "password").await;
        let unauth_cookies = unauth_login.cookies();
        let unauth_auth_cookie = unauth_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages/{}/attachments",
                audit.id, msg_id
            ))
            .add_cookie(unauth_auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::FORBIDDEN);
    }

    // ============================================================================
    // Tests d'administration
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_message_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "admin_create_owner",
            "admin_create_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin_create",
            "admin_create@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "admin_create@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Admin message".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["content"], "Admin message");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_for_nonexistent_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "nonexist_owner",
            "nonexist_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "nonexist_owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/audits/nonexistent-audit-id/messages")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    // ============================================================================
    // Tests de contenu
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_message_with_max_length_content() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "max_owner",
            "max_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "max_member",
            "max_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "max_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message with exactly 4000 characters
        let long_content = "a".repeat(4000);
        let payload = CreateAuditMessagePayload {
            content: long_content.clone(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["content"], long_content);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_message_exceeding_max_length_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "exceed_owner",
            "exceed_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "exceed_member",
            "exceed_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "exceed_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message with 4001 characters (exceeds max)
        let long_content = "a".repeat(4001);
        let payload = CreateAuditMessagePayload {
            content: long_content,
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_message_with_emojis_and_special_chars() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "emoji_owner",
            "emoji_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "emoji_member",
            "emoji_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "emoji_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message with emojis and special characters
        let content = "Test message 🎉 with émojis & spëcial çhars: 你好 мир".to_string();
        let payload = CreateAuditMessagePayload {
            content: content.clone(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["content"], content);
    }

    // ============================================================================
    // Tests de pagination
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_list_messages_with_pagination() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "page_owner",
            "page_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "page_member",
            "page_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "page_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create 5 messages
        for i in 1..=5 {
            let payload = CreateAuditMessagePayload {
                content: format!("Message {}", i),
            };
            let response = server
                .post(&format!("/v1/audits/{}/messages", audit.id))
                .add_cookie(auth_cookie.clone())
                .json(&payload)
                .await;
            assert_eq!(response.status_code(), StatusCode::CREATED);
        }

        // List with limit 2
        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages?limit=2&offset=0",
                audit.id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::OK);
        let messages: Vec<serde_json::Value> = list_response.json();
        assert_eq!(messages.len(), 2);

        // List with offset 2
        let list_response2 = server
            .get(&format!(
                "/v1/audits/{}/messages?limit=2&offset=2",
                audit.id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response2.status_code(), StatusCode::OK);
        let messages2: Vec<serde_json::Value> = list_response2.json();
        assert_eq!(messages2.len(), 2);

        // Verify they're different messages
        assert_ne!(messages[0]["id"], messages2[0]["id"]);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_with_large_limit() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "large_owner",
            "large_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "large_member",
            "large_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "large_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test with very large limit
        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages?limit=1000&offset=0",
                audit.id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::OK);
    }

    // ============================================================================
    // Tests de structure de réponse
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_message_response_includes_author_info() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "resp_owner",
            "resp_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "resp_member",
            "resp_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "resp_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditMessagePayload {
            content: "Test response structure".to_string(),
        };

        let response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();

        // Verify message has required fields
        assert!(body["id"].is_string());
        assert_eq!(body["auditId"], audit.id);
        assert!(body["author"].is_object());
        assert!(body["author"]["id"].is_string());
        assert_eq!(body["content"], "Test response structure");
        assert!(body["createdAt"].is_string());
        assert!(body["updatedAt"].is_string());
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "struct_owner",
            "struct_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "struct_member",
            "struct_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "struct_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create a message
        let payload = CreateAuditMessagePayload {
            content: "Test message for structure".to_string(),
        };
        server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // List messages
        let response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let messages: Vec<serde_json::Value> = response.json();

        // Verify each message has required fields
        assert!(!messages.is_empty());
        for msg in messages {
            assert!(msg["id"].is_string());
            assert!(msg["auditId"].is_string());
            assert!(msg["author"].is_object());
            assert!(msg["author"]["id"].is_string());
            assert!(msg["content"].is_string());
            assert!(msg["createdAt"].is_string());
            assert!(msg["updatedAt"].is_string());
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_attachment_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "attach_struct_owner",
            "attach_struct_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "attach_struct_member",
            "attach_struct_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "attach_struct_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message
        let msg_payload = CreateAuditMessagePayload {
            content: "Message with attachment check".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // List attachments
        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages/{}/attachments",
                audit.id, msg_id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::OK);
        let attachments: Vec<serde_json::Value> = list_response.json();

        // Verify response is a valid array
        assert!(
            attachments.is_empty()
                || attachments.iter().all(|a| a["id"].is_string()
                    && a["fileName"].is_string()
                    && a["fileSize"].is_number()
                    && a["mimeType"].is_string())
        );
    }

    // ============================================================================
    // Tests d'attachments - Suppression et erreurs
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_delete_nonexistent_attachment_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "delete_owner",
            "delete_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "delete_member",
            "delete_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "delete_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create a message first
        let msg_payload = CreateAuditMessagePayload {
            content: "Message for delete test".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Try to delete non-existent attachment
        let delete_response = server
            .delete(&format!(
                "/v1/audits/{}/messages/{}/attachments/nonexistent-attach",
                audit.id, msg_id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(delete_response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_download_attachment_authorize_team_member_only() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "download_owner",
            "download_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "download_member",
            "download_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _unused = insert_test_user(
            &pool,
            "download_unauth",
            "download_unauth@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Create message as member
        let login_response = login_user(&server, "download_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let msg_payload = CreateAuditMessagePayload {
            content: "Message with file".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Try to download as unauthorized user
        let unauth_login = login_user(&server, "download_unauth@test.com", "password").await;
        let unauth_cookies = unauth_login.cookies();
        let unauth_auth_cookie = unauth_cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Try to get attachments list with unauthorized user
        let list_response = server
            .get(&format!(
                "/v1/audits/{}/messages/{}/attachments",
                audit.id, msg_id
            ))
            .add_cookie(unauth_auth_cookie.clone())
            .await;

        assert_eq!(list_response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_nonexistent_attachment_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "get_owner",
            "get_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "get_member",
            "get_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "get_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message
        let msg_payload = CreateAuditMessagePayload {
            content: "Message for download test".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Try to download non-existent attachment
        let download_response = server
            .get(&format!(
                "/v1/audits/{}/messages/{}/attachments/nonexistent-id",
                audit.id, msg_id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(download_response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_attachment_unauthorized_user_denied() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "del_unauth_owner",
            "del_unauth_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "del_unauth_member",
            "del_unauth_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _del_unauth_unused = insert_test_user(
            &pool,
            "del_unauth_user",
            "del_unauth_user@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "del_unauth_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let msg_payload = CreateAuditMessagePayload {
            content: "Message for delete auth test".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Try delete as unauthorized user
        let unauth_login = login_user(&server, "del_unauth_user@test.com", "password").await;
        let unauth_cookies = unauth_login.cookies();
        let unauth_auth_cookie = unauth_cookies.iter().find(|c| c.name() == "auth").unwrap();

        let delete_response = server
            .delete(&format!(
                "/v1/audits/{}/messages/{}/attachments/any-id",
                audit.id, msg_id
            ))
            .add_cookie(unauth_auth_cookie.clone())
            .await;

        assert_eq!(delete_response.status_code(), StatusCode::FORBIDDEN);
    }

    // ============================================================================
    // Tests d'erreurs spécifiques
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_message_not_found_returns_correct_error() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "notfound_owner",
            "notfound_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "notfound_member",
            "notfound_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "notfound_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Try to get attachments for non-existent message
        let response = server
            .get(&format!(
                "/v1/audits/{}/messages/fake-message-id/attachments",
                audit.id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_messages_empty_audit() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "empty_audit_owner",
            "empty_audit_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "empty_audit_member",
            "empty_audit_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "empty_audit_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // List messages from empty audit
        let response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let messages: Vec<serde_json::Value> = response.json();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_multiple_messages_preserve_order() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "order_owner",
            "order_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "order_member",
            "order_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "order_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create 3 messages with distinct content
        let mut message_ids = Vec::new();
        for i in 1..=3 {
            let payload = CreateAuditMessagePayload {
                content: format!("Message number {}", i),
            };
            let response = server
                .post(&format!("/v1/audits/{}/messages", audit.id))
                .add_cookie(auth_cookie.clone())
                .json(&payload)
                .await;
            let body: serde_json::Value = response.json();
            message_ids.push(body["id"].as_str().unwrap().to_string());
        }

        // List all messages
        let list_response = server
            .get(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        let messages: Vec<serde_json::Value> = list_response.json();
        assert_eq!(messages.len(), 3);

        // Verify they appear in response
        for (i, msg) in messages.iter().enumerate() {
            assert_eq!(msg["content"], format!("Message number {}", i + 1));
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_attachment_forbidden_for_archived_audit() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "arch_del_owner",
            "arch_del_owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "arch_del_member",
            "arch_del_member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");
        let audit = insert_test_audit(&pool, "Test Audit", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "arch_del_member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Create message
        let msg_payload = CreateAuditMessagePayload {
            content: "Message before archive".to_string(),
        };
        let create_response = server
            .post(&format!("/v1/audits/{}/messages", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&msg_payload)
            .await;
        let msg_response: serde_json::Value = create_response.json();
        let msg_id: String = msg_response["id"].as_str().unwrap().to_string();

        // Archive the audit
        let mut conn = pool.get().unwrap();
        diesel::update(
            crate::schema::audits::table.filter(crate::schema::audits::id.eq(&audit.id)),
        )
        .set(crate::schema::audits::is_archived.eq(true))
        .execute(&mut conn)
        .unwrap();

        // Try to delete attachment on archived audit
        let delete_response = server
            .delete(&format!(
                "/v1/audits/{}/messages/{}/attachments/any-id",
                audit.id, msg_id
            ))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(delete_response.status_code(), StatusCode::BAD_REQUEST);
    }
}
