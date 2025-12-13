#[cfg(test)]
mod auth_integration_tests {
    use crate::api::auth::{LoginPayload, RegisterPayload, ConfirmEmailPayload, ResetPasswordRequestPayload, ResetPasswordPayload};
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        auth::{create_valid_email_token, create_expired_email_token, create_valid_reset_token, create_expired_reset_token},
        db::setup_test_db,
        fixtures::insert_test_user,
    };
    use axum_test::TestServer;
    use http::StatusCode;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_login_success() {
        // Setup: Démarrer PostgreSQL container et créer le test state
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur de test
        let _user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        // Créer le router et le serveur de test
        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Tenter de se connecter
        let payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/login").json(&payload).await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);

        // Vérifier qu'un cookie auth a été défini
        let cookies = response.cookies();
        assert!(
            cookies.iter().any(|c| c.name() == "auth"),
            "Auth cookie should be set"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_login_invalid_credentials() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur de test
        let _user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Tenter de se connecter avec un mauvais mot de passe
        let payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let response = server.post("/v1/auth/login").json(&payload).await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.text(),
            "\"error.user.auth.invalidCredentials\""
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_login_user_not_found() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Tenter de se connecter avec un email inexistant
        let payload = LoginPayload {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/login").json(&payload).await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.text(),
            "\"error.user.auth.invalidCredentials\""
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_login_invalid_email_format() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Tenter de se connecter avec un email invalide
        let payload = LoginPayload {
            email: "not-an-email".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/login").json(&payload).await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.user.login.invalidEmail\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_logout() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur et se connecter
        let _user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter d'abord
        let login_payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(login_response.status_code(), StatusCode::OK);

        // Récupérer le cookie auth
        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Se déconnecter
        let logout_response = server
            .post("/v1/auth/logout")
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(logout_response.status_code(), StatusCode::OK);

        // Vérifier que le cookie auth a été supprimé/expiré
        let cookies = logout_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth");
        assert!(auth_cookie.is_some(), "Auth cookie should be present");

        // Le cookie devrait être vide (valeur vide = suppression)
        assert_eq!(
            auth_cookie.unwrap().value(),
            "",
            "Auth cookie should be empty after logout"
        );
    }

    // ============================================================================
    // Tests pour POST /v1/auth/register
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_register_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = RegisterPayload {
            username: "newuser".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            email: "newuser@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/register").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_register_invalid_email() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = RegisterPayload {
            username: "newuser".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            email: "not-an-email".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/register").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.user.register.invalidEmail\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_register_duplicate_username() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur existant
        insert_test_user(
            &pool,
            "existinguser",
            "existing@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = RegisterPayload {
            username: "existinguser".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            email: "different@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/register").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
        assert!(response.text().contains("username"));
    }

    #[tokio::test]
    #[serial]
    async fn test_register_duplicate_email() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur existant
        insert_test_user(
            &pool,
            "existinguser",
            "existing@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = RegisterPayload {
            username: "differentuser".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            email: "existing@example.com".to_string(),
            password: "password123".to_string(),
        };

        let response = server.post("/v1/auth/register").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
        assert!(response.text().contains("email"));
    }

    // ============================================================================
    // Tests pour POST /v1/auth/confirm-email
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_confirm_email_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Créer un token de confirmation valide
        let token = create_valid_email_token(
            "newuser",
            "New",
            "User",
            "newuser@example.com",
            "password123",
        );

        let payload = ConfirmEmailPayload { token };

        let response = server.post("/v1/auth/confirm-email").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        assert_eq!(response.text(), "\"success.auth.confirmEmail\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_confirm_email_invalid_token() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ConfirmEmailPayload {
            token: "invalid_token_that_cannot_be_decrypted".to_string(),
        };

        let response = server.post("/v1/auth/confirm-email").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.auth.confirmEmail.invalidToken\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_confirm_email_expired_token() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Créer un token expiré
        let token = create_expired_email_token(
            "newuser",
            "New",
            "User",
            "newuser@example.com",
            "password123",
        );

        let payload = ConfirmEmailPayload { token };

        let response = server.post("/v1/auth/confirm-email").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.auth.confirmEmail.invalidToken\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_confirm_email_duplicate_username() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur existant avec le même username
        insert_test_user(
            &pool,
            "existinguser",
            "different@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Token avec le même username
        let token = create_valid_email_token(
            "existinguser",
            "New",
            "User",
            "newuser@example.com",
            "password123",
        );

        let payload = ConfirmEmailPayload { token };

        let response = server.post("/v1/auth/confirm-email").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
        assert!(response.text().contains("username"));
    }

    #[tokio::test]
    #[serial]
    async fn test_confirm_email_duplicate_email() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur existant avec le même email
        insert_test_user(
            &pool,
            "differentuser",
            "existing@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Token avec le même email
        let token = create_valid_email_token(
            "newuser",
            "New",
            "User",
            "existing@example.com",
            "password123",
        );

        let payload = ConfirmEmailPayload { token };

        let response = server.post("/v1/auth/confirm-email").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::CONFLICT);
        assert!(response.text().contains("email"));
    }

    // ============================================================================
    // Tests pour POST /v1/auth/request-password-reset
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_request_reset_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordRequestPayload {
            reset_email: "test@example.com".to_string(),
        };

        let response = server.post("/v1/auth/request-password-reset").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_request_reset_nonexistent_user() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordRequestPayload {
            reset_email: "nonexistent@example.com".to_string(),
        };

        let response = server.post("/v1/auth/request-password-reset").json(&payload).await;

        // Devrait retourner 200 même si l'utilisateur n'existe pas (sécurité)
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    // ============================================================================
    // Tests pour POST /v1/auth/reset-password
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_reset_password_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "oldpassword",
            UserRole::Pentester,
        );

        // Créer un token de reset valide
        let token = create_valid_reset_token(&pool, &user.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordPayload {
            token,
            new_password: "newpassword123".to_string(),
        };

        let response = server.post("/v1/auth/reset-password").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::OK);

        // Vérifier qu'on peut se connecter avec le nouveau mot de passe
        let login_payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "newpassword123".to_string(),
        };

        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(login_response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_reset_password_invalid_token() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordPayload {
            token: "invalid_token_that_does_not_exist".to_string(),
            new_password: "newpassword123".to_string(),
        };

        let response = server.post("/v1/auth/reset-password").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.auth.resetPassword.invalidToken\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_reset_password_expired_token() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        // Créer un token expiré
        let token = create_expired_reset_token(&pool, &user.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordPayload {
            token,
            new_password: "newpassword123".to_string(),
        };

        let response = server.post("/v1/auth/reset-password").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text(), "\"error.auth.resetPassword.expiredToken\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_reset_password_token_consumed() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        // Créer un token de reset valide
        let token = create_valid_reset_token(&pool, &user.id);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let payload = ResetPasswordPayload {
            token: token.clone(),
            new_password: "newpassword123".to_string(),
        };

        // Premier reset - devrait réussir
        let response = server.post("/v1/auth/reset-password").json(&payload).await;
        assert_eq!(response.status_code(), StatusCode::OK);

        // Deuxième reset avec le même token - devrait échouer
        let payload2 = ResetPasswordPayload {
            token,
            new_password: "anotherpassword".to_string(),
        };

        let response2 = server.post("/v1/auth/reset-password").json(&payload2).await;
        assert_eq!(response2.status_code(), StatusCode::BAD_REQUEST);
    }

    // ============================================================================
    // Tests de flows complets
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_full_registration_flow() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Étape 1: Enregistrement
        let register_payload = RegisterPayload {
            username: "flowuser".to_string(),
            first_name: "Flow".to_string(),
            last_name: "User".to_string(),
            email: "flowuser@example.com".to_string(),
            password: "password123".to_string(),
        };

        let register_response = server.post("/v1/auth/register").json(&register_payload).await;
        assert_eq!(register_response.status_code(), StatusCode::OK);

        // Étape 2: Confirmation email (simuler avec un token créé)
        let token = create_valid_email_token(
            "flowuser",
            "Flow",
            "User",
            "flowuser@example.com",
            "password123",
        );

        let confirm_payload = ConfirmEmailPayload { token };

        let confirm_response = server.post("/v1/auth/confirm-email").json(&confirm_payload).await;
        assert_eq!(confirm_response.status_code(), StatusCode::CREATED);

        // Étape 3: Login avec le compte créé
        let login_payload = LoginPayload {
            email: "flowuser@example.com".to_string(),
            password: "password123".to_string(),
        };

        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(login_response.status_code(), StatusCode::OK);

        // Vérifier que le cookie auth est défini
        let cookies = login_response.cookies();
        assert!(cookies.iter().any(|c| c.name() == "auth"));
    }

    #[tokio::test]
    #[serial]
    async fn test_full_password_reset_flow() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        let user = insert_test_user(
            &pool,
            "resetuser",
            "reset@example.com",
            "oldpassword",
            UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Étape 1: Demander un reset de mot de passe
        let request_payload = ResetPasswordRequestPayload {
            reset_email: "reset@example.com".to_string(),
        };

        let request_response = server.post("/v1/auth/request-password-reset").json(&request_payload).await;
        assert_eq!(request_response.status_code(), StatusCode::OK);

        // Étape 2: Reset le mot de passe avec un token (simuler avec un token créé)
        let token = create_valid_reset_token(&pool, &user.id);

        let reset_payload = ResetPasswordPayload {
            token,
            new_password: "newpassword123".to_string(),
        };

        let reset_response = server.post("/v1/auth/reset-password").json(&reset_payload).await;
        assert_eq!(reset_response.status_code(), StatusCode::OK);

        // Étape 3: Login avec le nouveau mot de passe
        let login_payload = LoginPayload {
            email: "reset@example.com".to_string(),
            password: "newpassword123".to_string(),
        };

        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(login_response.status_code(), StatusCode::OK);

        // Vérifier que l'ancien mot de passe ne fonctionne plus
        let old_login_payload = LoginPayload {
            email: "reset@example.com".to_string(),
            password: "oldpassword".to_string(),
        };

        let old_login_response = server.post("/v1/auth/login").json(&old_login_payload).await;
        assert_eq!(old_login_response.status_code(), StatusCode::UNAUTHORIZED);
    }
}
