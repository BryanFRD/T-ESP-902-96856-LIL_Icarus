#[cfg(test)]
mod status_integration_tests {
    use crate::tests::helpers::{
        app::{create_test_router, create_test_state},
        db::setup_test_db,
    };
    use axum_test::TestServer;
    use http::StatusCode;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_get_status_success() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert!(
            body.get("status").is_some(),
            "Status field should be present"
        );
        assert!(
            body.get("message").is_some(),
            "Message field should be present"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_returns_correct_format() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();

        let status = body.get("status").expect("Status field should exist");
        let message = body.get("message").expect("Message field should exist");

        assert!(
            status == "Up" || status == "Maintenance",
            "Status should be 'Up' or 'Maintenance'"
        );
        assert!(message.is_string(), "Message should be a string");
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_up() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();

        let status = body.get("status").expect("Status field should exist");
        let message = body.get("message").expect("Message field should exist");

        assert_eq!(status, "Up", "Status should be 'Up' in test environment");
        assert!(
            !message.as_str().unwrap().is_empty(),
            "Message should not be empty"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_no_authentication_required() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(
            response.status_code(),
            StatusCode::OK,
            "Status endpoint should be accessible without authentication"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_response_structure() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();

        assert!(body.is_object(), "Response should be a JSON object");
        assert_eq!(
            body.as_object().unwrap().len(),
            2,
            "Response should have exactly 2 fields: status and message"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_multiple_requests() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        for _ in 0..5 {
            let response = server.get("/v1/status").await;

            assert_eq!(
                response.status_code(),
                StatusCode::OK,
                "All status requests should succeed"
            );

            let body: serde_json::Value = response.json();
            assert!(body.get("status").is_some());
            assert!(body.get("message").is_some());
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_get_status_content_type() {
        let (_pg, pool) = setup_test_db().await;
        let database_url = "test_db".to_string();
        let state = create_test_state(pool, database_url);

        let app = create_test_router(state);
        let server = TestServer::new(app);

        let response = server.get("/v1/status").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let content_type = response
            .headers()
            .get("content-type")
            .expect("Content-Type header should be present");

        assert!(
            content_type.to_str().unwrap().contains("application/json"),
            "Content-Type should be application/json"
        );
    }
}
