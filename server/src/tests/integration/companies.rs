#[cfg(test)]
mod companies_integration_tests {
    use crate::api::auth::LoginPayload;
    use crate::api::teams::companies::{CreateCompanyPayload, UpdateCompanyPayload};
    use crate::api::teams::TeamRole;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{
            create_team_with_owner, insert_team_member, insert_test_company,
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

    #[tokio::test]
    #[serial]
    async fn test_create_company_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        // Créer un utilisateur owner d'une équipe
        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        // Se connecter
        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies
            .iter()
            .find(|c| c.name() == "auth")
            .expect("Auth cookie should be set");

        // Test: Créer une entreprise
        let payload = CreateCompanyPayload {
            name: "Test Company".to_string(),
            description: Some("A test company".to_string()),
            siret: "12345678901234".to_string(),
        };

        let response = server
            .post(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "Test Company");
        assert_eq!(body["description"], "A test company");
        assert_eq!(body["siret"], "12345678901234");
        assert_eq!(body["teamId"], team.id);
        assert!(body["id"].is_string());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_company_duplicate_siret() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);

        // Créer une première entreprise
        insert_test_company(&pool, &team.id, "Company 1", None, "12345678901234");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Essayer de créer une entreprise avec le même SIRET
        let payload = CreateCompanyPayload {
            name: "Company 2".to_string(),
            description: None,
            siret: "12345678901234".to_string(),
        };

        let response = server
            .post(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_company_unauthorized_member() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let member = insert_test_user(&pool, "member", "member@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &owner);

        // Ajouter le member comme simple membre (pas owner/moderator)
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Le membre ne devrait pas pouvoir créer une entreprise
        let payload = CreateCompanyPayload {
            name: "Test Company".to_string(),
            description: None,
            siret: "12345678901234".to_string(),
        };

        let response = server
            .post(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_companies() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);

        // Créer 3 entreprises
        insert_test_company(&pool, &team.id, "Company 1", None, "11111111111111");
        insert_test_company(&pool, &team.id, "Company 2", Some("Desc 2".to_string()), "22222222222222");
        insert_test_company(&pool, &team.id, "Company 3", None, "33333333333333");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Lister les entreprises
        let response = server
            .get(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        let body: Vec<serde_json::Value> = response.json();
        assert_eq!(body.len(), 3);
        assert_eq!(body[0]["name"], "Company 1");
        assert_eq!(body[1]["name"], "Company 2");
        assert_eq!(body[2]["name"], "Company 3");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_companies_unauthorized() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let _other_user = insert_test_user(&pool, "other", "other@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &owner);

        insert_test_company(&pool, &team.id, "Company 1", None, "11111111111111");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "other@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Essayer de lister les entreprises sans être membre
        let response = server
            .get(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_company_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);
        let company = insert_test_company(&pool, &team.id, "Test Company", Some("Description".to_string()), "12345678901234");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Récupérer l'entreprise
        let response = server
            .get(&format!("/v1/teams/{}/company/{}", team.id, company.id))
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["id"], company.id);
        assert_eq!(body["name"], "Test Company");
        assert_eq!(body["description"], "Description");
        assert_eq!(body["siret"], "12345678901234");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_company_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);
        let company = insert_test_company(&pool, &team.id, "Old Name", None, "12345678901234");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Mettre à jour l'entreprise
        let payload = UpdateCompanyPayload {
            name: Some("New Name".to_string()),
            description: Some("New Description".to_string()),
            siret: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/company/{}", team.id, company.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::OK);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "New Name");
        assert_eq!(body["description"], "New Description");
        assert_eq!(body["siret"], "12345678901234"); // Inchangé
    }

    #[tokio::test]
    #[serial]
    async fn test_update_company_unauthorized() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let member = insert_test_user(&pool, "member", "member@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &owner);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");

        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Le membre ne devrait pas pouvoir modifier
        let payload = UpdateCompanyPayload {
            name: Some("New Name".to_string()),
            description: None,
            siret: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/company/{}", team.id, company.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_company_success() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);
        let company = insert_test_company(&pool, &team.id, "Company", None, "12345678901234");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Supprimer l'entreprise
        let response = server
            .delete(&format!("/v1/teams/{}/company/{}", team.id, company.id))
            .add_cookie(auth_cookie.clone())
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Vérifier que l'entreprise n'existe plus
        let get_response = server
            .get(&format!("/v1/teams/{}/company/{}", team.id, company.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(get_response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_moderator_can_manage_companies() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let moderator = insert_test_user(&pool, "moderator", "moderator@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &owner);

        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "moderator@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Le modérateur devrait pouvoir créer une entreprise
        let payload = CreateCompanyPayload {
            name: "Moderator Company".to_string(),
            description: None,
            siret: "99999999999999".to_string(),
        };

        let response = server
            .post(&format!("/v1/teams/{}/company", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        // Assertions
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    #[tokio::test]
    #[serial]
    async fn test_cascade_delete_companies_when_team_deleted() {
        // Setup
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(&pool, "owner", "owner@test.com", "password", &UserRole::Pentester);
        let team = create_team_with_owner(&pool, &user);

        // Créer plusieurs entreprises
        let company1 = insert_test_company(&pool, &team.id, "Company 1", None, "11111111111111");
        let company2 = insert_test_company(&pool, &team.id, "Company 2", None, "22222222222222");

        let app = create_test_router(state);
        let server = TestServer::new(app).expect("Failed to create test server");

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        // Test: Supprimer l'équipe
        let delete_team_response = server
            .delete(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(delete_team_response.status_code(), StatusCode::NO_CONTENT);

        // Vérifier que les entreprises n'existent plus
        let get_company1 = server
            .get(&format!("/v1/teams/{}/company/{}", team.id, company1.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(get_company1.status_code(), StatusCode::NOT_FOUND);

        let get_company2 = server
            .get(&format!("/v1/teams/{}/company/{}", team.id, company2.id))
            .add_cookie(auth_cookie.clone())
            .await;
        assert_eq!(get_company2.status_code(), StatusCode::NOT_FOUND);
    }
}
