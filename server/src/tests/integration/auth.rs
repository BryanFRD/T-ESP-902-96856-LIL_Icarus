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

    // ============================================================================
    // Tests de middleware
    // ============================================================================

    use crate::api::auth::{admin_middleware, middleware, pentester_middleware, JwtClaims};
    use axum::{Json, Router, middleware::from_fn, response::IntoResponse, routing::get};

    // Endpoints de test pour les middlewares
    async fn protected_endpoint(jwt: JwtClaims) -> impl IntoResponse {
        Json(serde_json::json!({
            "user_id": jwt.sub,
            "email": jwt.email,
            "role": jwt.role,
        }))
    }

    async fn admin_endpoint(_jwt: JwtClaims) -> impl IntoResponse {
        Json("admin_ok")
    }

    async fn pentester_endpoint(_jwt: JwtClaims) -> impl IntoResponse {
        Json("pentester_ok")
    }

    // Router de test avec endpoints protégés par middlewares
    fn create_middleware_test_router(state: crate::state::GlobalState) -> Router {
        use crate::api::{auth, status, teams, users};
        use tower_cookies::CookieManagerLayer;

        // Créer les routes de test pour les middlewares avec leur state
        let protected_route = Router::new()
            .route("/test/protected", get(protected_endpoint))
            .route_layer(from_fn(middleware))
            .with_state(state.clone());

        let admin_route = Router::new()
            .route("/test/admin", get(admin_endpoint))
            .route_layer(from_fn(admin_middleware))
            .route_layer(from_fn(middleware))
            .with_state(state.clone());

        let pentester_route = Router::new()
            .route("/test/pentester", get(pentester_endpoint))
            .route_layer(from_fn(pentester_middleware))
            .route_layer(from_fn(middleware))
            .with_state(state.clone());

        // Créer le router complet avec les routes de test ET les routes d'API
        let router_v1 = Router::new()
            .nest("/teams", teams::get_routes(&state))
            .nest("/users", users::get_routes(&state))
            .nest("/auth", auth::get_routes(state.clone()))
            .nest("/status", status::get_routes(state.clone()))
            .merge(protected_route)
            .merge(admin_route)
            .merge(pentester_route);

        Router::new()
            .nest("/v1", router_v1)
            .layer(CookieManagerLayer::new())
    }

    // ============================================================================
    // Tests pour middleware() - Authentification basique
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_middleware_allows_valid_jwt() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Set JWT_SECRET env var (required by middleware)
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter pour obtenir le cookie auth
        let login_payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint protégé avec un JWT valide
        let response = server
            .get("/v1/test/protected")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["email"], "test@example.com");
    }

    #[tokio::test]
    #[serial]
    async fn test_middleware_rejects_missing_cookie() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder sans cookie auth
        let response = server.get("/v1/test/protected").await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_middleware_rejects_invalid_jwt() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder avec un JWT invalide
        let invalid_cookie = tower_cookies::Cookie::build(("auth", "invalid_jwt_token"))
            .path("/")
            .http_only(true)
            .build();

        let response = server
            .get("/v1/test/protected")
            .add_cookie(invalid_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_middleware_rejects_expired_jwt() {
        use chrono::{Duration, Utc};
        use crate::tests::helpers::app::create_test_secrets;

        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url.clone());

        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        // Créer un JWT expiré manuellement
        let secrets = create_test_secrets(database_url);
        let expired_claims = JwtClaims {
            sub: ulid::Ulid::from_string(&user.id).unwrap(),
            email: user.email,
            role: user.role,
            iat: usize::try_from((Utc::now() - Duration::hours(25)).timestamp()).unwrap_or(0),
            exp: usize::try_from((Utc::now() - Duration::hours(1)).timestamp()).unwrap_or(0), // Expiré il y a 1 heure
        };
        let expired_token = expired_claims.to_string(&secrets.jwt_secret).unwrap();
        let expired_cookie = tower_cookies::Cookie::build(("auth", expired_token))
            .path("/")
            .http_only(true)
            .build();

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder avec un JWT expiré
        let response = server
            .get("/v1/test/protected")
            .add_cookie(expired_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_middleware_rejects_jwt_with_wrong_secret() {
        use chrono::{Duration, Utc};

        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            UserRole::Pentester,
        );

        // Créer un JWT avec un mauvais secret
        let wrong_secret = "wrong_secret_key_123456789";
        let claims = JwtClaims {
            sub: ulid::Ulid::from_string(&user.id).unwrap(),
            email: user.email,
            role: user.role,
            iat: usize::try_from(Utc::now().timestamp()).unwrap_or(0),
            exp: usize::try_from((Utc::now() + Duration::hours(24)).timestamp()).unwrap_or(0),
        };
        let wrong_token = claims.to_string(wrong_secret).unwrap();
        let wrong_cookie = tower_cookies::Cookie::build(("auth", wrong_token))
            .path("/")
            .http_only(true)
            .build();

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder avec un JWT signé avec le mauvais secret
        let response = server
            .get("/v1/test/protected")
            .add_cookie(wrong_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }

    // ============================================================================
    // Tests pour admin_middleware() - Vérification du rôle admin
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_admin_middleware_allows_admin() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "adminuser",
            "admin@example.com",
            "password123",
            UserRole::Admin,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant qu'admin
        let login_payload = LoginPayload {
            email: "admin@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint admin
        let response = server
            .get("/v1/test/admin")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "\"admin_ok\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_admin_middleware_rejects_pentester() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "pentesteruser",
            "pentester@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant que pentester
        let login_payload = LoginPayload {
            email: "pentester@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint admin en tant que pentester
        let response = server
            .get("/v1/test/admin")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_admin_middleware_rejects_client() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "clientuser",
            "client@example.com",
            "password123",
            UserRole::Client,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant que client
        let login_payload = LoginPayload {
            email: "client@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint admin en tant que client
        let response = server
            .get("/v1/test/admin")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_admin_middleware_rejects_unauthenticated() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder sans authentification
        let response = server.get("/v1/test/admin").await;

        // Assertions: Le middleware de base rejette avant admin_middleware
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }

    // ============================================================================
    // Tests pour pentester_middleware() - Vérification du rôle pentester
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_pentester_middleware_allows_pentester() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "pentesteruser",
            "pentester@example.com",
            "password123",
            UserRole::Pentester,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant que pentester
        let login_payload = LoginPayload {
            email: "pentester@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint pentester
        let response = server
            .get("/v1/test/pentester")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "\"pentester_ok\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_pentester_middleware_allows_admin() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "adminuser",
            "admin@example.com",
            "password123",
            UserRole::Admin,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant qu'admin
        let login_payload = LoginPayload {
            email: "admin@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint pentester en tant qu'admin (admin est aussi pentester)
        let response = server
            .get("/v1/test/pentester")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "\"pentester_ok\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_pentester_middleware_rejects_client() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let _user = insert_test_user(
            &pool,
            "clientuser",
            "client@example.com",
            "password123",
            UserRole::Client,
        );

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter en tant que client
        let login_payload = LoginPayload {
            email: "client@example.com".to_string(),
            password: "password123".to_string(),
        };
        let login_response = server.post("/v1/auth/login").json(&login_payload).await;
        let auth_cookie = login_response
            .cookies()
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set")
            .clone();

        // Test: Accéder à un endpoint pentester en tant que client
        let response = server
            .get("/v1/test/pentester")
            .add_cookie(auth_cookie)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_pentester_middleware_rejects_unauthenticated() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);
        unsafe { std::env::set_var("JWT_SECRET", "test_jwt_secret_key_for_testing_only_12345"); }

        let app = create_middleware_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Test: Accéder sans authentification
        let response = server.get("/v1/test/pentester").await;

        // Assertions: Le middleware de base rejette avant pentester_middleware
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.text(), "\"error.user.auth.unauthorized\"");
    }
}
