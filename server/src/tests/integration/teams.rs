#[cfg(test)]
mod teams_integration_tests {
    use crate::api::auth::LoginPayload;
    use crate::api::teams::TeamRole;
    use crate::api::teams::models::{CreateTeam, UpdateMemberRolePayload, UpdateTeam};
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{create_team_with_owner, insert_team_member, insert_test_user},
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
    // Tests pour POST /v1/teams/me - Créer une équipe
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_create_team_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let _user = insert_test_user(
            &pool,
            "user",
            "user@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = CreateTeam {
            name: "Test Team".to_string(),
            description: "A test team".to_string(),
        };

        let response = server
            .post("/v1/teams/me")
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::CREATED);
        let body: serde_json::Value = response.json();
        assert_eq!(body["name"], "Test Team");
        assert_eq!(body["description"], "A test team");
        assert!(body["id"].is_string());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_team_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = CreateTeam {
            name: "Test Team".to_string(),
            description: "A test team".to_string(),
        };

        let response = server.post("/v1/teams/me").json(&payload).await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour GET /v1/teams/me - Récupérer les équipes
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_get_my_teams_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "user",
            "user@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _team1 = create_team_with_owner(&pool, &user);
        let _team2 = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/teams/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: Vec<serde_json::Value> = response.json();
        assert_eq!(body.len(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_my_teams_empty() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let _user = insert_test_user(
            &pool,
            "user",
            "user@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .get("/v1/teams/me")
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        let body: Vec<serde_json::Value> = response.json();
        assert_eq!(body.len(), 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_my_teams_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/teams/me").await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour PATCH /v1/teams/{team_id} - Mettre à jour une équipe
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_update_team_success_as_owner() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: Some("Updated Team".to_string()),
            description: Some("Updated description".to_string()),
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_success_as_admin() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin",
            "admin@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "admin@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: Some("Admin Updated".to_string()),
            description: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_no_fields_provided() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: None,
            description: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response.text(),
            "\"error.team.update.validate.noFieldProvided\""
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_not_found() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let _user = insert_test_user(
            &pool,
            "user",
            "user@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: Some("Updated".to_string()),
            description: None,
        };

        let fake_team_id = ulid::Ulid::new().to_string();
        let response = server
            .patch(&format!("/v1/teams/{fake_team_id}"))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "\"error.team.notFound\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_forbidden_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: Some("Updated".to_string()),
            description: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_forbidden_non_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _other = insert_test_user(
            &pool,
            "other",
            "other@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "other@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateTeam {
            name: Some("Updated".to_string()),
            description: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_team_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = UpdateTeam {
            name: Some("Updated".to_string()),
            description: None,
        };

        let response = server
            .patch(&format!("/v1/teams/{}", team.id))
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour DELETE /v1/teams/{team_id} - Supprimer une équipe
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_delete_team_success_as_owner() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_team_success_as_admin() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin",
            "admin@test.com",
            "password",
            &UserRole::Admin,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "admin@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_team_not_found() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let _user = insert_test_user(
            &pool,
            "user",
            "user@test.com",
            "password",
            &UserRole::Pentester,
        );

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "user@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_team_id = ulid::Ulid::new().to_string();
        let response = server
            .delete(&format!("/v1/teams/{fake_team_id}"))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "\"error.team.notFound\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_team_forbidden_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_team_forbidden_non_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _other = insert_test_user(
            &pool,
            "other",
            "other@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "other@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}", team.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_team_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let user = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &user);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.delete(&format!("/v1/teams/{}", team.id)).await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour DELETE /v1/teams/{team_id}/members/{member_id} - Supprimer un membre
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_remove_member_success_as_owner() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_success_as_moderator() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator",
            "moderator@test.com",
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
        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "moderator@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_success_self_removal() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "member@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_success_as_admin() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin",
            "admin@test.com",
            "password",
            &UserRole::Admin,
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "admin@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_not_found() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let fake_member_id = ulid::Ulid::new().to_string();
        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, fake_member_id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "\"error.team.member.notFound\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_cannot_remove_owner() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator",
            "moderator@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "moderator@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, owner.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.team.member.cannotRemoveOwner\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_forbidden_as_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member1 = insert_test_user(
            &pool,
            "member1",
            "member1@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member2 = insert_test_user(
            &pool,
            "member2",
            "member2@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member1.id, TeamRole::Member);
        insert_team_member(&pool, &team.id, &member2.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "member1@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member2.id))
            .add_cookie(auth_cookie.clone())
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_remove_member_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server
            .delete(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }

    // ============================================================================
    // Tests pour PATCH /v1/teams/{team_id}/members/{member_id} - Mettre à jour le rôle
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_success_as_owner() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Moderator,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_success_as_admin() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let _admin = insert_test_user(
            &pool,
            "admin",
            "admin@test.com",
            "password",
            &UserRole::Admin,
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "admin@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Moderator,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_not_found() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "owner@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Moderator,
        };

        let fake_member_id = ulid::Ulid::new().to_string();
        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, fake_member_id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "\"error.team.member.notFound\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_forbidden_as_moderator() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let moderator = insert_test_user(
            &pool,
            "moderator",
            "moderator@test.com",
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
        insert_team_member(&pool, &team.id, &moderator.id, TeamRole::Moderator);
        insert_team_member(&pool, &team.id, &member.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "moderator@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Owner,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_forbidden_as_member() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member1 = insert_test_user(
            &pool,
            "member1",
            "member1@test.com",
            "password",
            &UserRole::Pentester,
        );
        let member2 = insert_test_user(
            &pool,
            "member2",
            "member2@test.com",
            "password",
            &UserRole::Pentester,
        );
        let team = create_team_with_owner(&pool, &owner);
        insert_team_member(&pool, &team.id, &member1.id, TeamRole::Member);
        insert_team_member(&pool, &team.id, &member2.id, TeamRole::Member);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let login_response = login_user(&server, "member1@test.com", "password").await;
        let cookies = login_response.cookies();
        let auth_cookie = cookies.iter().find(|c| c.name() == "auth").unwrap();

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Moderator,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, member2.id))
            .add_cookie(auth_cookie.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(response.text(), "\"error.user.auth.forbidden\"");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_member_role_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool.clone(), database_url);

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
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

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let payload = UpdateMemberRolePayload {
            role: TeamRole::Moderator,
        };

        let response = server
            .patch(&format!("/v1/teams/{}/members/{}", team.id, member.id))
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    }
}
