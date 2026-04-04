#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod agents_integration_tests {
    use crate::api::agents::CreateAgentPayload;
    use crate::api::auth::LoginPayload;
    use crate::api::users::UserRole;
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
        fixtures::{create_complete_audit_setup, insert_test_user},
    };
    use axum_test::TestServer;
    use http::StatusCode;
    use serial_test::serial;

    async fn setup_authenticated() -> (TestServer, String) {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());

        let owner = insert_test_user(
            &pool,
            "owner",
            "owner@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::builder()
            .save_cookies()
            .build(app);

        let login_payload = LoginPayload {
            email: "owner@test.com".to_string(),
            password: "password".to_string(),
        };
        let resp = server.post("/v1/auth/login").json(&login_payload).await;
        assert_eq!(resp.status_code(), StatusCode::OK);

        // Step-up
        let step_up = serde_json::json!({ "password": "password" });
        server
            .post("/v1/audits/verify-access")
            .json(&step_up)
            .await;

        (server, audit.id)
    }

    #[tokio::test]
    #[serial]
    async fn test_create_agent() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateAgentPayload {
            name: "agent-01".into(),
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["name"], "agent-01");
        assert!(json["token"].as_str().is_some());
        assert!(!json["token"].as_str().unwrap().is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_agent_duplicate_name() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateAgentPayload {
            name: "agent-dup".into(),
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);

        // Same name again
        let resp2 = server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;
        assert_eq!(resp2.status_code(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_agents() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateAgentPayload {
            name: "agent-list".into(),
        };
        server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;

        let resp = server
            .get(&format!("/v1/audits/{audit_id}/agents"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert!(!json.as_array().unwrap().is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_revoke_agent() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateAgentPayload {
            name: "agent-revoke".into(),
        };
        let create_resp = server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;
        let agent: serde_json::Value = create_resp.json();
        let agent_id = agent["id"].as_str().unwrap();

        let resp = server
            .delete(&format!("/v1/audits/{audit_id}/agents/{agent_id}"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_revoke_agent_already_revoked() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateAgentPayload {
            name: "agent-double-revoke".into(),
        };
        let create_resp = server
            .post(&format!("/v1/audits/{audit_id}/agents"))
            .json(&payload)
            .await;
        let agent: serde_json::Value = create_resp.json();
        let agent_id = agent["id"].as_str().unwrap();

        server
            .delete(&format!("/v1/audits/{audit_id}/agents/{agent_id}"))
            .await;

        let resp2 = server
            .delete(&format!("/v1/audits/{audit_id}/agents/{agent_id}"))
            .await;
        assert_ne!(resp2.status_code(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    #[serial]
    async fn test_revoke_agent_not_found() {
        let (server, audit_id) = setup_authenticated().await;
        let resp = server
            .delete(&format!("/v1/audits/{audit_id}/agents/nonexistent"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_agents_unauthenticated() {
        let (_pg, pool) = setup_test_db().await;
        let state = create_test_state(pool.clone(), "test_db".to_string());
        let owner = insert_test_user(
            &pool,
            "owner2",
            "owner2@test.com",
            "password",
            &UserRole::Pentester,
        );
        let (_team, _company, audit) = create_complete_audit_setup(&pool, &owner);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let resp = server
            .get(&format!("/v1/audits/{}/agents", audit.id))
            .await;
        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }
}
