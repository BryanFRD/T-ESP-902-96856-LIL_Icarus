#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tickets_integration_tests {
    use crate::api::audits::tickets::columns::models::CreateColumnPayload;
    use crate::api::audits::tickets::comments::models::{
        CreateCommentPayload, UpdateCommentPayload,
    };
    use crate::api::audits::tickets::labels::models::CreateLabelPayload;
    use crate::api::audits::tickets::models::payload::{
        CreateTicketPayload, MoveTicketPayload, UpdateTicketPayload,
    };
    use crate::api::audits::tickets::models::priority::TicketPriority;
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

        // Login
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

    async fn create_column(server: &TestServer, audit_id: &str, name: &str) -> String {
        let payload = CreateColumnPayload {
            name: name.to_string(),
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/columns"))
            .json(&payload)
            .await;
        let json: serde_json::Value = resp.json();
        json["id"].as_str().unwrap().to_string()
    }

    async fn create_ticket(
        server: &TestServer,
        audit_id: &str,
        column_id: &str,
        title: &str,
    ) -> serde_json::Value {
        let payload = CreateTicketPayload {
            title: title.to_string(),
            description: None,
            priority: None,
            column_id: column_id.to_string(),
            assignee_id: None,
            parent_id: None,
            due_date: None,
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/tickets"))
            .json(&payload)
            .await;
        resp.json()
    }

    // ======= Column CRUD =======

    #[tokio::test]
    #[serial]
    async fn test_create_column() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateColumnPayload {
            name: "To Do".into(),
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/columns"))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["name"], "To Do");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_columns() {
        let (server, audit_id) = setup_authenticated().await;
        create_column(&server, &audit_id, "Done").await;

        let resp = server
            .get(&format!("/v1/audits/{audit_id}/columns"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert!(!json.as_array().unwrap().is_empty());
    }

    // ======= Ticket CRUD =======

    #[tokio::test]
    #[serial]
    async fn test_create_ticket() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "Backlog").await;

        let payload = CreateTicketPayload {
            title: "First ticket".into(),
            description: Some("Description".into()),
            priority: Some(TicketPriority::High),
            column_id: col_id,
            assignee_id: None,
            parent_id: None,
            due_date: None,
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/tickets"))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["title"], "First ticket");
        assert_eq!(json["priority"], "HIGH");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_tickets() {
        let (server, audit_id) = setup_authenticated().await;
        let resp = server
            .get(&format!("/v1/audits/{audit_id}/tickets"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_ticket() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "Test").await;
        let ticket = create_ticket(&server, &audit_id, &col_id, "Get me").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let resp = server
            .get(&format!("/v1/audits/{audit_id}/tickets/{ticket_id}"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["title"], "Get me");
    }

    #[tokio::test]
    #[serial]
    async fn test_update_ticket() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "Col").await;
        let ticket = create_ticket(&server, &audit_id, &col_id, "Original").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let update = UpdateTicketPayload {
            title: Some("Updated".into()),
            description: Some("New desc".into()),
            priority: Some(TicketPriority::Critical),
            assignee_id: None,
            clear_assignee: false,
            parent_id: None,
            clear_parent: false,
            due_date: None,
            clear_due_date: false,
        };
        let resp = server
            .patch(&format!("/v1/audits/{audit_id}/tickets/{ticket_id}"))
            .json(&update)
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["title"], "Updated");
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_ticket() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "Del").await;
        let ticket = create_ticket(&server, &audit_id, &col_id, "To delete").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let resp = server
            .delete(&format!("/v1/audits/{audit_id}/tickets/{ticket_id}"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::NO_CONTENT);
    }

    // ======= Move ticket =======

    #[tokio::test]
    #[serial]
    async fn test_move_ticket() {
        let (server, audit_id) = setup_authenticated().await;
        let col1 = create_column(&server, &audit_id, "Col1").await;
        let col2 = create_column(&server, &audit_id, "Col2").await;
        let ticket = create_ticket(&server, &audit_id, &col1, "Moveable").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let move_payload = MoveTicketPayload {
            column_id: col2,
            position: 0,
        };
        let resp = server
            .put(&format!(
                "/v1/audits/{audit_id}/tickets/{ticket_id}/move"
            ))
            .json(&move_payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    // ======= Board =======

    #[tokio::test]
    #[serial]
    async fn test_get_board() {
        let (server, audit_id) = setup_authenticated().await;
        let resp = server
            .get(&format!("/v1/audits/{audit_id}/board"))
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["auditId"], audit_id);
        assert!(json["columns"].is_array());
    }

    // ======= Labels =======

    #[tokio::test]
    #[serial]
    async fn test_create_and_list_labels() {
        let (server, audit_id) = setup_authenticated().await;
        let payload = CreateLabelPayload {
            name: "Bug".into(),
            color: None,
        };
        let resp = server
            .post(&format!("/v1/audits/{audit_id}/labels"))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);

        let list_resp = server
            .get(&format!("/v1/audits/{audit_id}/labels"))
            .await;
        assert_eq!(list_resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = list_resp.json();
        assert!(!json.as_array().unwrap().is_empty());
    }

    // ======= Comments =======

    #[tokio::test]
    #[serial]
    async fn test_create_and_list_comments() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "Cmts").await;
        let ticket = create_ticket(&server, &audit_id, &col_id, "With comments").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let payload = CreateCommentPayload {
            content: "Nice work".into(),
        };
        let resp = server
            .post(&format!(
                "/v1/audits/{audit_id}/tickets/{ticket_id}/comments"
            ))
            .json(&payload)
            .await;
        assert_eq!(resp.status_code(), StatusCode::CREATED);

        let list_resp = server
            .get(&format!(
                "/v1/audits/{audit_id}/tickets/{ticket_id}/comments"
            ))
            .await;
        assert_eq!(list_resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_comment() {
        let (server, audit_id) = setup_authenticated().await;
        let col_id = create_column(&server, &audit_id, "UC").await;
        let ticket = create_ticket(&server, &audit_id, &col_id, "T").await;
        let ticket_id = ticket["id"].as_str().unwrap();

        let create_resp = server
            .post(&format!(
                "/v1/audits/{audit_id}/tickets/{ticket_id}/comments"
            ))
            .json(&CreateCommentPayload {
                content: "Original".into(),
            })
            .await;
        let comment: serde_json::Value = create_resp.json();
        let comment_id = comment["id"].as_str().unwrap();

        let resp = server
            .patch(&format!(
                "/v1/audits/{audit_id}/tickets/{ticket_id}/comments/{comment_id}"
            ))
            .json(&UpdateCommentPayload {
                content: "Edited".into(),
            })
            .await;
        assert_eq!(resp.status_code(), StatusCode::OK);
        let json: serde_json::Value = resp.json();
        assert_eq!(json["content"], "Edited");
    }

    // ======= Unauthenticated access =======

    #[tokio::test]
    #[serial]
    async fn test_tickets_unauthenticated() {
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
            .get(&format!("/v1/audits/{}/tickets", audit.id))
            .await;
        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }
}
