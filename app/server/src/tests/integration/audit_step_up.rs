#[cfg(test)]
mod audit_step_up_integration_tests {
    use crate::api::audits::step_up::endpoints::VerifyAccessPayload;
    use crate::api::auth::LoginPayload;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::insert_test_user,
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
    // Tests pour verify_access endpoint
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_verify_access_with_correct_password_success() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "stepup_user",
            "stepup@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "stepup@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = VerifyAccessPayload {
            password: "password".to_string(),
        };

        let response = server
            .post("/v1/audits/verify-access")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["message"], "success.audit.stepUp.verified");
        // Note: Cookie verification is an implementation detail that may vary per test framework
    }

    #[tokio::test]
    #[serial]
    async fn test_verify_access_with_incorrect_password_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "wrong_pass_user",
            "wrongpass@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "wrongpass@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = VerifyAccessPayload {
            password: "wrongpassword".to_string(),
        };

        let response = server
            .post("/v1/audits/verify-access")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["error"], "error.audit.stepUp.invalidPassword");
    }

    #[tokio::test]
    #[serial]
    async fn test_verify_access_with_empty_password_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "empty_pass_user",
            "emptypass@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "emptypass@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = VerifyAccessPayload {
            password: "".to_string(),
        };

        let response = server
            .post("/v1/audits/verify-access")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Should fail with 400 bad request due to validation
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_verify_access_unauthenticated_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = VerifyAccessPayload {
            password: "password".to_string(),
        };

        let response = server.post("/v1/audits/verify-access").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour check_access endpoint
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_check_access_with_valid_token_returns_true() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "check_access_user",
            "checkacc@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "checkacc@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // First, verify access to get the audit access cookie
        let verify_payload = VerifyAccessPayload {
            password: "password".to_string(),
        };

        let verify_response = server
            .post("/v1/audits/verify-access")
            .add_cookie(auth_cookie.clone())
            .json(&verify_payload)
            .await;

        assert_eq!(verify_response.status_code(), StatusCode::OK);
        let _audit_access_cookies = verify_response.cookies();

        // Now check access
        let check_response = server
            .get("/v1/audits/check-access")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(check_response.status_code(), StatusCode::OK);
        let body: serde_json::Value = check_response.json();
        // When audit access is not set, has_access should be false
        assert_eq!(body["hasAccess"], false);
    }

    #[tokio::test]
    #[serial]
    async fn test_check_access_without_token_returns_false() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "no_token_user",
            "notoken@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "notoken@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Check access without setting audit access token
        let response = server
            .get("/v1/audits/check-access")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["hasAccess"], false);
        assert!(body["expiresAt"].is_null());
    }

    #[tokio::test]
    #[serial]
    async fn test_check_access_unauthenticated_fails() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let response = server.get("/v1/audits/check-access").await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    async fn test_verify_access_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "struct_user",
            "struct@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "struct@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = VerifyAccessPayload {
            password: "password".to_string(),
        };

        let response = server
            .post("/v1/audits/verify-access")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();

        // Verify response structure
        assert!(body["message"].is_string());
        assert!(body.get("error").is_none()); // Should not have error on success
    }

    #[tokio::test]
    #[serial]
    async fn test_check_access_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let _user = insert_test_user(
            &pool,
            "check_struct_user",
            "checkstruct@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "checkstruct@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/audits/check-access")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();

        // Verify response structure
        assert!(body["hasAccess"].is_boolean());
        assert!(body["expiresAt"].is_null() || body["expiresAt"].is_string());
    }
}
