use crate::api::ws::error::WsError;

#[test]
fn test_ws_error_display_auth_failed() {
    let err = WsError::AuthFailed("bad token".to_string());
    assert_eq!(err.to_string(), "Authentication failed: bad token");
}

#[test]
fn test_ws_error_display_room_access_denied() {
    let err = WsError::RoomAccessDenied("not a member".to_string());
    assert_eq!(err.to_string(), "Room access denied: not a member");
}

#[test]
fn test_ws_error_display_step_up_required() {
    assert_eq!(
        WsError::StepUpRequired.to_string(),
        "Step-up authentication required"
    );
}

#[test]
fn test_ws_error_display_step_up_invalid() {
    assert_eq!(
        WsError::StepUpInvalid.to_string(),
        "Step-up authentication invalid"
    );
}

#[test]
fn test_ws_error_display_invalid_payload() {
    let err = WsError::InvalidPayload("missing field".to_string());
    assert_eq!(err.to_string(), "Invalid payload: missing field");
}

#[test]
fn test_ws_error_display_database_error() {
    assert_eq!(WsError::DatabaseError.to_string(), "Database error");
}

#[test]
fn test_ws_error_display_audit_not_found() {
    assert_eq!(WsError::AuditNotFound.to_string(), "Audit not found");
}

#[test]
fn test_ws_error_display_audit_archived() {
    assert_eq!(WsError::AuditArchived.to_string(), "Audit is archived");
}

#[test]
fn test_ws_error_code_auth_failed() {
    assert_eq!(
        WsError::AuthFailed("x".into()).code(),
        "AUTH_FAILED"
    );
}

#[test]
fn test_ws_error_code_room_access_denied() {
    assert_eq!(
        WsError::RoomAccessDenied("x".into()).code(),
        "ROOM_ACCESS_DENIED"
    );
}

#[test]
fn test_ws_error_code_step_up_required() {
    assert_eq!(WsError::StepUpRequired.code(), "AUDIT_STEP_UP_REQUIRED");
}

#[test]
fn test_ws_error_code_step_up_invalid() {
    assert_eq!(WsError::StepUpInvalid.code(), "AUDIT_STEP_UP_INVALID");
}

#[test]
fn test_ws_error_code_invalid_payload() {
    assert_eq!(
        WsError::InvalidPayload("x".into()).code(),
        "INVALID_PAYLOAD"
    );
}

#[test]
fn test_ws_error_code_database_error() {
    assert_eq!(WsError::DatabaseError.code(), "DATABASE_ERROR");
}

#[test]
fn test_ws_error_code_audit_not_found() {
    assert_eq!(WsError::AuditNotFound.code(), "AUDIT_NOT_FOUND");
}

#[test]
fn test_ws_error_code_audit_archived() {
    assert_eq!(WsError::AuditArchived.code(), "AUDIT_ARCHIVED");
}

#[test]
fn test_ws_error_to_response() {
    let err = WsError::AuthFailed("bad creds".to_string());
    let resp = err.to_response();
    assert_eq!(resp.code, "AUTH_FAILED");
    assert_eq!(resp.message, "Authentication failed: bad creds");
}

#[test]
fn test_ws_error_response_serialization() {
    let err = WsError::DatabaseError;
    let resp = err.to_response();
    let json = serde_json::to_value(&resp).unwrap();
    assert_eq!(json["code"], "DATABASE_ERROR");
    assert_eq!(json["message"], "Database error");
}
