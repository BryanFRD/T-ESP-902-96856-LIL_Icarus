#[cfg(test)]
mod users_integration_tests {
    use crate::api::auth::LoginPayload;
    use crate::api::users::{UpdatePayload, UserRole};
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::insert_test_user,
    };
    use axum_test::TestServer;
    use http::StatusCode;
    use serial_test::serial;

    /// Helper pour authentifier un utilisateur et retourner le serveur avec le cookie auth
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

    #[tokio::test]
    #[serial]
    async fn test_get_me_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        let user = insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Se connecter
        let login_response = login_user(&server, "test@example.com", "password123").await;

        // Récupérer le cookie auth
        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Récupérer les infos de l'utilisateur connecté
        let response = server
            .get("/v1/users/me")
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["username"], user.username);
        assert_eq!(body["email"], user.email);
        assert_eq!(body["firstName"], user.first_name);
        assert_eq!(body["lastName"], user.last_name);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_me_unauthorized() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Test: Essayer de récupérer les infos sans authentification
        let response = server.get("/v1/users/me").await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_me_username() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Se connecter
        let login_response = login_user(&server, "test@example.com", "password123").await;

        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Mettre à jour le username
        let payload = UpdatePayload {
            username: Some("newusername".to_string()),
            first_name: None,
            last_name: None,
            email: None,
            password: None,
            new_password: None,
            confirm_password: None,
        };

        let response = server
            .patch("/v1/users/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);

        // Vérifier que le username a bien été mis à jour
        let get_response = server
            .get("/v1/users/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(get_response.status_code(), StatusCode::OK);
        let body: serde_json::Value = get_response.json();
        assert_eq!(body["username"], "newusername");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_me_email() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Se connecter
        let login_response = login_user(&server, "test@example.com", "password123").await;

        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Mettre à jour l'email
        let payload = UpdatePayload {
            username: None,
            first_name: None,
            last_name: None,
            email: Some("newemail@example.com".to_string()),
            password: None,
            new_password: None,
            confirm_password: None,
        };

        let response = server
            .patch("/v1/users/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);

        // Le cookie devrait être mis à jour avec le nouveau email
        let new_cookies = response.cookies();
        assert!(
            new_cookies.iter().any(|c| c.name() == "auth"),
            "New auth cookie should be set after email update"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_update_me_unauthorized() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Test: Essayer de mettre à jour sans authentification
        let payload = UpdatePayload {
            username: Some("newusername".to_string()),
            first_name: None,
            last_name: None,
            email: None,
            password: None,
            new_password: None,
            confirm_password: None,
        };

        let response = server.patch("/v1/users/me").json(&payload).await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_me_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur
        insert_test_user(
            &pool,
            "testuser",
            "test@example.com",
            "password123",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Se connecter
        let login_response = login_user(&server, "test@example.com", "password123").await;

        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Supprimer l'utilisateur
        let response = server
            .delete("/v1/users/me")
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Vérifier que l'utilisateur ne peut plus se connecter
        let login_payload = LoginPayload {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let login_after_delete = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(login_after_delete.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_me_unauthorized() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        // Test: Essayer de supprimer sans authentification
        let response = server.delete("/v1/users/me").await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }
}
