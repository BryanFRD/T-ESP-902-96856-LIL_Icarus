#[cfg(test)]
mod audits_integration_tests {
    use crate::api::audits::{CreateAuditPayload, UpdateAuditPayload};
    use crate::api::auth::LoginPayload;
    use crate::api::teams::TeamRole;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{
            create_complete_audit_setup, create_team_with_owner, insert_audit_client,
            insert_team_member, insert_test_audit, insert_test_company, insert_test_user,
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
    // Tests de permissions - Création
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_audit_as_team_owner_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company",
            Some("Desc".to_string()),
            "12345678901234",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test description".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "Test Audit");
        assert_eq!(body["teamId"], team.id);
        assert!(body["company"].is_object());
        assert_eq!(body["company"]["id"], company.id);
        assert_eq!(body["company"]["name"], company.name);
        assert_eq!(body["isArchived"], false);
        assert!(body["clients"].is_array());
        assert_eq!(body["clients"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner50",
            "owner50@test.com",
            "password",
            &UserRole::Pentester,
        );
        let admin = insert_test_user(
            &pool,
            "admin50",
            "admin50@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);

        // L'admin doit être dans la team pour pouvoir créer un audit
        insert_team_member(&pool, &team.id, &admin.id, TeamRole::Member);

        let company = insert_test_company(
            &pool,
            &team.id,
            "Company50",
            Some("Desc".to_string()),
            "50505050505050",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "admin50@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "Admin Audit".to_string(),
            description: Some("Test by admin".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "Admin Audit");
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_invalid_team_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner51",
            "owner51@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company51",
            Some("Desc".to_string()),
            "51515151515151",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner51@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_team_id = ulid::Ulid::new().to_string();
        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test".to_string()),
            team_id: fake_team_id,
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_invalid_company_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner52",
            "owner52@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner52@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_company_id = ulid::Ulid::new().to_string();
        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test".to_string()),
            team_id: team.id.clone(),
            company_id: fake_company_id,
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_company_from_different_team_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner1 = insert_test_user(
            &pool,
            "owner53",
            "owner53@test.com",
            "password",
            &UserRole::Pentester,
        );
        let owner2 = insert_test_user(
            &pool,
            "owner54",
            "owner54@test.com",
            "password",
            &UserRole::Pentester,
        );

        let team1 = create_team_with_owner(&pool, &owner1);
        let team2 = create_team_with_owner(&pool, &owner2);

        let company2 = insert_test_company(
            &pool,
            &team2.id,
            "Company54",
            Some("Desc".to_string()),
            "54545454545454",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner53@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de créer un audit dans team1 avec une company de team2
        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test".to_string()),
            team_id: team1.id.clone(),
            company_id: company2.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_as_moderator_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner55",
            "owner55@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator55",
            "moderator55@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company55",
            Some("Desc".to_string()),
            "55555555555555",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "moderator55@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "Forbidden Audit".to_string(),
            description: None,
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_as_member_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner3",
            "owner3@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "member",
            "member@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company3",
            Some("Desc".to_string()),
            "11111111111111",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "Forbidden Audit".to_string(),
            description: None,
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_audits_only_from_user_teams() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner1 = insert_test_user(
            &pool,
            "owner10",
            "owner10@test.com",
            "password",
            &UserRole::Pentester,
        );
        let owner2 = insert_test_user(
            &pool,
            "owner11",
            "owner11@test.com",
            "password",
            &UserRole::Pentester,
        );

        let (team1, _company1, audit1) = create_complete_audit_setup(&pool, &owner1);
        let (_team2, _company2, _audit2) = create_complete_audit_setup(&pool, &owner2);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner10@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert!(body.is_array());
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["id"], audit1.id);
        assert_eq!(audits[0]["teamId"], team1.id);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_audit_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner60",
            "owner60@test.com",
            "password",
            &UserRole::Pentester,
        );
        let admin = insert_test_user(
            &pool,
            "admin60",
            "admin60@test.com",
            "password",
            &UserRole::Admin,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // L'admin doit être dans la team
        insert_team_member(&pool, &team.id, &admin.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "admin60@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateAuditPayload {
            name: Some("Updated by Admin".to_string()),
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "Updated by Admin");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_audit_as_moderator_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner61",
            "owner61@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator61",
            "moderator61@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "moderator61@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateAuditPayload {
            name: Some("Should Fail".to_string()),
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_nonexistent_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner62",
            "owner62@test.com",
            "password",
            &UserRole::Pentester,
        );
        create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner62@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_audit_id = ulid::Ulid::new().to_string();
        let payload = UpdateAuditPayload {
            name: Some("Should Fail".to_string()),
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", fake_audit_id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_archive_audit_as_owner_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner18",
            "owner18@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner18@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Verify it's archived
        let get_response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        let body: serde_json::Value = get_response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["isArchived"], true);
    }

    #[tokio::test]
    #[serial]
    async fn test_archive_audit_as_admin_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner70",
            "owner70@test.com",
            "password",
            &UserRole::Pentester,
        );
        let admin = insert_test_user(
            &pool,
            "admin70",
            "admin70@test.com",
            "password",
            &UserRole::Admin,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // L'admin doit être dans la team
        insert_team_member(&pool, &team.id, &admin.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "admin70@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Verify it's archived
        let get_response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        let body: serde_json::Value = get_response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["isArchived"], true);
    }

    #[tokio::test]
    #[serial]
    async fn test_archive_audit_as_moderator_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner71",
            "owner71@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator71",
            "moderator71@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "moderator71@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_archive_nonexistent_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner72",
            "owner72@test.com",
            "password",
            &UserRole::Pentester,
        );
        create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner72@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_audit_id = ulid::Ulid::new().to_string();
        let response = server
            .delete(&format!("/v1/audits/{}", fake_audit_id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    // ============================================================================
    // Tests d'archivage automatique
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_auto_archive_on_get_with_expired_end_date() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner20",
            "owner20@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company20",
            Some("Desc".to_string()),
            "20202020202020",
        );

        // Créer un audit avec une end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let audit = insert_test_audit(
            &pool,
            "Expired Audit",
            Some("Should be auto-archived".to_string()),
            &team.id,
            &company.id,
            Some(past_date),
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner20@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // GET devrait auto-archiver l'audit
        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["id"], audit.id);
        assert_eq!(audits[0]["isArchived"], true);
    }

    #[tokio::test]
    #[serial]
    async fn test_auto_archive_on_patch_returns_403() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner21",
            "owner21@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company21",
            Some("Desc".to_string()),
            "21212121212121",
        );

        // Créer un audit avec une end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let audit = insert_test_audit(
            &pool,
            "Expired Audit",
            Some("Should be auto-archived".to_string()),
            &team.id,
            &company.id,
            Some(past_date),
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner21@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // PATCH devrait auto-archiver puis retourner 403
        let payload = UpdateAuditPayload {
            name: Some("New Name".to_string()),
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_auto_archive_on_delete_returns_403() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner22",
            "owner22@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company22",
            Some("Desc".to_string()),
            "22222222222222",
        );

        // Créer un audit avec une end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let audit = insert_test_audit(
            &pool,
            "Expired Audit",
            Some("Should be auto-archived".to_string()),
            &team.id,
            &company.id,
            Some(past_date),
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner22@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // DELETE devrait auto-archiver puis retourner 403
        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_patch_manually_archived_audit_returns_403() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner23",
            "owner23@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner23@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Archive manuellement
        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Tenter de PATCH
        let payload = UpdateAuditPayload {
            name: Some("New Name".to_string()),
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_manually_archived_audit_returns_403() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner24",
            "owner24@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner24@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Archive manuellement
        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Tenter de DELETE à nouveau
        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_audit_with_future_end_date_not_archived() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner25",
            "owner25@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company25",
            Some("Desc".to_string()),
            "25252525252525",
        );

        // Créer un audit avec une end_date dans le futur
        let future_date = chrono::Utc::now().naive_utc() + chrono::Duration::days(30);
        let audit = insert_test_audit(
            &pool,
            "Future Audit",
            Some("Should not be archived".to_string()),
            &team.id,
            &company.id,
            Some(future_date),
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner25@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // GET ne devrait PAS archiver l'audit
        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["id"], audit.id);
        assert_eq!(audits[0]["isArchived"], false);
    }

    #[tokio::test]
    #[serial]
    async fn test_audit_without_end_date_never_archived() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner26",
            "owner26@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner26@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // GET ne devrait PAS archiver l'audit (pas de end_date)
        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0]["id"], audit.id);
        assert_eq!(audits[0]["isArchived"], false);

        // Attendre un peu et revérifier
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits[0]["isArchived"], false);
    }

    #[tokio::test]
    #[serial]
    async fn test_batch_archiving_multiple_expired_audits() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner27",
            "owner27@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company27",
            Some("Desc".to_string()),
            "27272727272727",
        );

        // Créer plusieurs audits avec end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let audit1 = insert_test_audit(
            &pool,
            "Expired 1",
            None,
            &team.id,
            &company.id,
            Some(past_date),
        );
        let audit2 = insert_test_audit(
            &pool,
            "Expired 2",
            None,
            &team.id,
            &company.id,
            Some(past_date),
        );
        let audit3 = insert_test_audit(
            &pool,
            "Expired 3",
            None,
            &team.id,
            &company.id,
            Some(past_date),
        );

        // Créer un audit sans end_date
        let audit4 = insert_test_audit(&pool, "No end date", None, &team.id, &company.id, None);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner27@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // GET devrait archiver tous les audits expirés en un seul batch
        let response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        let audits = body.as_array().unwrap();
        assert_eq!(audits.len(), 4);

        // Vérifier que les 3 premiers sont archivés
        let archived_count = audits.iter().filter(|a| a["isArchived"] == true).count();
        assert_eq!(archived_count, 3);

        // Vérifier que le 4ème n'est pas archivé
        let not_archived = audits.iter().find(|a| a["id"] == audit4.id).unwrap();
        assert_eq!(not_archived["isArchived"], false);

        // Vérifier que les 3 expirés sont bien archivés
        for audit in [&audit1, &audit2, &audit3] {
            let found = audits.iter().find(|a| a["id"] == audit.id).unwrap();
            assert_eq!(found["isArchived"], true);
        }
    }

    // ============================================================================
    // Tests de validation
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_past_end_date_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner30",
            "owner30@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company30",
            Some("Desc".to_string()),
            "30303030303030",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner30@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de créer avec une end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: Some(past_date),
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_audit_with_past_end_date_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner31",
            "owner31@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner31@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de mettre à jour avec une end_date dans le passé
        let past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let payload = UpdateAuditPayload {
            name: None,
            description: None,
            end_date: Some(past_date),
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_null_end_date_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner32",
            "owner32@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company32",
            Some("Desc".to_string()),
            "32323232323232",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner32@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "Test Audit".to_string(),
            description: Some("Test".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert!(body["endDate"].is_null());
    }

    #[tokio::test]
    #[serial]
    async fn test_update_audit_remove_end_date_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner33",
            "owner33@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company33",
            Some("Desc".to_string()),
            "33333333333333",
        );

        // Créer un audit avec une end_date
        let future_date = chrono::Utc::now().naive_utc() + chrono::Duration::days(30);
        let audit = insert_test_audit(
            &pool,
            "Test Audit",
            None,
            &team.id,
            &company.id,
            Some(future_date),
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner33@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Retirer la end_date
        let payload = UpdateAuditPayload {
            name: None,
            description: None,
            end_date: None,
            clear_end_date: true,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert!(body["endDate"].is_null());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_empty_name_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner34",
            "owner34@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company34",
            Some("Desc".to_string()),
            "34343434343434",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner34@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateAuditPayload {
            name: "".to_string(),
            description: Some("Test".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_audit_with_long_name_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner35",
            "owner35@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(
            &pool,
            &team.id,
            "Company35",
            Some("Desc".to_string()),
            "35353535353535",
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner35@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Créer un nom de 256 caractères
        let long_name = "a".repeat(256);
        let payload = CreateAuditPayload {
            name: long_name,
            description: Some("Test".to_string()),
            team_id: team.id.clone(),
            company_id: company.id.clone(),
            end_date: None,
        };

        let response = server
            .post("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_audit_with_no_fields_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner36",
            "owner36@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner36@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de mettre à jour sans aucun champ
        let payload = UpdateAuditPayload {
            name: None,
            description: None,
            end_date: None,
            clear_end_date: false,
        };

        let response = server
            .patch(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    // ============================================================================
    // Tests de gestion des clients
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_team_owner_can_remove_client() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner40",
            "owner40@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client40",
            "client40@test.com",
            "password",
            &UserRole::Client,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner40@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Retirer le client
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Vérifier que le client n'est plus dans l'audit
        let get_response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        let body: serde_json::Value = get_response.json();
        let audits = body.as_array().unwrap();
        let audit_response = audits.iter().find(|a| a["id"] == audit.id).unwrap();
        let clients = audit_response["clients"].as_array().unwrap();
        assert_eq!(clients.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_admin_can_remove_client() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner41",
            "owner41@test.com",
            "password",
            &UserRole::Pentester,
        );
        let admin = insert_test_user(
            &pool,
            "admin41",
            "admin41@test.com",
            "password",
            &UserRole::Admin,
        );
        let client = insert_test_user(
            &pool,
            "client41",
            "client41@test.com",
            "password",
            &UserRole::Client,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter l'admin à la team (en tant que member)
        insert_team_member(&pool, &team.id, &admin.id, TeamRole::Member);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "admin41@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Retirer le client en tant qu'admin
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_member_cannot_remove_client() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner42",
            "owner42@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member = insert_test_user(
            &pool,
            "member42",
            "member42@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client42",
            "client42@test.com",
            "password",
            &UserRole::Client,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter le member à la team
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "member42@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de retirer le client
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_moderator_cannot_remove_client() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner43",
            "owner43@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator43",
            "moderator43@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client43",
            "client43@test.com",
            "password",
            &UserRole::Client,
        );
        let (team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter le moderator à la team
        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "moderator43@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de retirer le client
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_client_not_in_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner44",
            "owner44@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client44",
            "client44@test.com",
            "password",
            &UserRole::Client,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ne PAS ajouter le client à l'audit

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner44@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de retirer un client qui n'est pas dans l'audit
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_client_from_archived_audit_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner45",
            "owner45@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client45",
            "client45@test.com",
            "password",
            &UserRole::Client,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner45@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Archiver l'audit d'abord
        let response = server
            .delete(&format!("/v1/audits/{}", audit.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Tenter de retirer le client d'un audit archivé
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_non_owner_cannot_remove_client() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner46",
            "owner46@test.com",
            "password",
            &UserRole::Pentester,
        );
        let other_owner = insert_test_user(
            &pool,
            "other46",
            "other46@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client46",
            "client46@test.com",
            "password",
            &UserRole::Client,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Créer une autre team pour other_owner
        create_team_with_owner(&pool, &other_owner);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "other46@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Tenter de retirer le client d'un audit dont on n'est pas owner
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_client_removal_deletes_from_database() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner47",
            "owner47@test.com",
            "password",
            &UserRole::Pentester,
        );
        let client = insert_test_user(
            &pool,
            "client47",
            "client47@test.com",
            "password",
            &UserRole::Client,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        // Ajouter le client à l'audit
        insert_audit_client(&pool, &audit.id, &client.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner47@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Vérifier que le client est dans l'audit
        let get_response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        let body: serde_json::Value = get_response.json();
        let audits = body.as_array().unwrap();
        let audit_response = audits.iter().find(|a| a["id"] == audit.id).unwrap();
        let clients = audit_response["clients"].as_array().unwrap();
        assert_eq!(clients.len(), 1);

        // Retirer le client
        let response = server
            .delete(&format!("/v1/audits/{}/clients/{}", audit.id, client.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Vérifier que le client a été retiré de la base de données
        let get_response = server
            .get("/v1/audits/me")
            .add_cookie(auth_cookie.clone())
            .await;

        let body: serde_json::Value = get_response.json();
        let audits = body.as_array().unwrap();
        let audit_response = audits.iter().find(|a| a["id"] == audit.id).unwrap();
        let clients = audit_response["clients"].as_array().unwrap();
        assert_eq!(clients.len(), 0);
    }
}
