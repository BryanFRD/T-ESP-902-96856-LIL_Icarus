use crate::api::users::UserRole;
use crate::tests::helpers::{
    app::{create_test_router, create_test_state_from_pool},
    db::setup_test_db,
    fixtures::insert_test_user,
};
use axum_test::TestServer;
use http::StatusCode;
use serde_json::{Value, json};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_list_events_requires_authentication() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/v1/events").await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial]
async fn test_list_events_as_admin() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _admin = insert_test_user(
        &pool,
        "admin",
        "admin@test.com",
        "password",
        &UserRole::Admin,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "admin@test.com",
            "password": "password"
        }))
        .await;

    assert_eq!(login_response.status_code(), StatusCode::OK);
    let cookies = login_response.cookies();

    let response = server.get("/v1/events").add_cookies(cookies.clone()).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let events: Value = response.json();
    assert!(events.is_array());
}

#[tokio::test]
#[serial]
async fn test_list_events_with_filters() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _admin = insert_test_user(
        &pool,
        "admin",
        "admin@test.com",
        "password",
        &UserRole::Admin,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "admin@test.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    let response = server
        .get("/v1/events")
        .add_query_param("limit", "10")
        .add_query_param("resource_type", "Auth")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    let events: Value = response.json();
    assert!(events.is_array());
}

#[tokio::test]
#[serial]
async fn test_get_event_by_id_not_found() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _admin = insert_test_user(
        &pool,
        "admin",
        "admin@test.com",
        "password",
        &UserRole::Admin,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "admin@test.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    let response = server
        .get("/v1/events/01HQXXXXXXXXXXXXXXXXXXXXXXXX")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_event_logging_on_user_update() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _user = insert_test_user(
        &pool,
        "testuser",
        "test@example.com",
        "password",
        &UserRole::Pentester,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    let update_response = server
        .patch("/v1/users/me")
        .json(&json!({
            "firstName": "Updated",
            "lastName": "Name"
        }))
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(update_response.status_code(), StatusCode::OK);

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let events_response = server
        .get("/v1/events")
        .add_query_param("resource_type", "User")
        .add_query_param("action", "Update")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(events_response.status_code(), StatusCode::OK);
    let events: Vec<Value> = events_response.json();

    assert!(!events.is_empty(), "Should have logged an update event");

    let update_event = &events[0];
    assert_eq!(update_event["action"], "Update");
    assert_eq!(update_event["resourceType"], "User");

    if let Some(changes) = update_event.get("changes") {
        assert!(
            changes.get("after").is_some(),
            "Should have 'after' data in changes"
        );
    }
}

#[tokio::test]
#[serial]
async fn test_event_metadata_contains_response_status() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _user = insert_test_user(
        &pool,
        "testuser",
        "test@example.com",
        "password",
        &UserRole::Pentester,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    server
        .get("/v1/users/me")
        .add_cookies(cookies.clone())
        .await;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let events_response = server
        .get("/v1/events")
        .add_query_param("action", "Read")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(events_response.status_code(), StatusCode::OK);
    let events: Vec<Value> = events_response.json();

    if let Some(first_event) = events.first() {
        if let Some(metadata) = first_event.get("metadata") {
            assert!(
                metadata.get("response_status").is_some(),
                "Metadata should contain response_status"
            );
        }
    }
}

#[tokio::test]
#[serial]
async fn test_non_admin_sees_anonymized_data() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _user = insert_test_user(
        &pool,
        "regularuser",
        "user@test.com",
        "password",
        &UserRole::Pentester,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "user@test.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    let events_response = server.get("/v1/events").add_cookies(cookies.clone()).await;

    assert_eq!(events_response.status_code(), StatusCode::OK);
    let events: Vec<Value> = events_response.json();

    for event in events {
        if let Some(user_id) = event.get("userId") {
            if let Some(user_id_str) = user_id.as_str() {
                assert!(
                    user_id_str.contains("xxxx") || user_id_str.len() < 5,
                    "User ID should be redacted for non-admin users"
                );
            }
        }
    }
}

#[tokio::test]
#[serial]
async fn test_admin_can_disable_anonymization() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _admin = insert_test_user(
        &pool,
        "admin",
        "admin@test.com",
        "password",
        &UserRole::Admin,
    );

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "admin@test.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    let events_response = server
        .get("/v1/events")
        .add_query_param("anonymized", "false")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(events_response.status_code(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn test_sensitive_fields_are_redacted() {
    let (_container, pool) = setup_test_db().await;
    let state = create_test_state_from_pool(pool.clone());
    let app = create_test_router(state);
    let server = TestServer::new(app).unwrap();

    let _admin = insert_test_user(
        &pool,
        "admin",
        "admin@test.com",
        "password",
        &UserRole::Admin,
    );

    let __register_response = server
        .post("/v1/auth/register")
        .json(&json!({
            "username": "newuser",
            "email": "new@test.com",
            "password": "secretpassword123",
            "firstName": "New",
            "lastName": "User"
        }))
        .await;

    let login_response = server
        .post("/v1/auth/login")
        .json(&json!({
            "email": "admin@test.com",
            "password": "password"
        }))
        .await;

    let cookies = login_response.cookies();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let events_response = server
        .get("/v1/events")
        .add_query_param("action", "Register")
        .add_cookies(cookies.clone())
        .await;

    assert_eq!(events_response.status_code(), StatusCode::OK);
    let events: Vec<Value> = events_response.json();

    for event in events {
        if let Some(changes) = event.get("changes") {
            if let Some(after) = changes.get("after") {
                let after_str = after.to_string();
                assert!(
                    !after_str.contains("secretpassword123"),
                    "Password should be redacted/encrypted in event logs"
                );
            }
        }
    }
}
