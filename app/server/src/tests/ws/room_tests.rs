use crate::api::ws::rooms::types::{Room, RoomPayload};

#[test]
fn test_room_payload_me_deserialization() {
    let json = r#"{"type": "me"}"#;
    let payload: RoomPayload = serde_json::from_str(json).unwrap();
    assert!(matches!(payload, RoomPayload::Me));
}

#[test]
fn test_room_payload_audit_deserialization() {
    let json = r#"{"type": "audit", "id": "abc123"}"#;
    let payload: RoomPayload = serde_json::from_str(json).unwrap();
    match payload {
        RoomPayload::Audit { id } => assert_eq!(id, "abc123"),
        RoomPayload::Me => panic!("Expected Audit variant"),
    }
}

#[test]
fn test_room_payload_me_serialization() {
    let payload = RoomPayload::Me;
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["type"], "me");
}

#[test]
fn test_room_payload_audit_serialization() {
    let payload = RoomPayload::Audit {
        id: "xyz".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["type"], "audit");
    assert_eq!(json["id"], "xyz");
}

#[test]
fn test_room_from_payload_me() {
    let payload = RoomPayload::Me;
    let room = Room::from_payload(&payload, "user123");
    assert!(matches!(room, Room::User(ref id) if id == "user123"));
}

#[test]
fn test_room_from_payload_audit() {
    let payload = RoomPayload::Audit {
        id: "audit456".to_string(),
    };
    let room = Room::from_payload(&payload, "user123");
    assert!(matches!(room, Room::Audit(ref id) if id == "audit456"));
}

#[test]
fn test_room_id_user() {
    let room = Room::User("u1".to_string());
    assert_eq!(room.room_id(), "user:u1");
}

#[test]
fn test_room_id_audit() {
    let room = Room::Audit("a1".to_string());
    assert_eq!(room.room_id(), "audit:a1");
}

#[test]
fn test_room_display_name_user() {
    let room = Room::User("u1".to_string());
    assert_eq!(room.display_name(), "me");
}

#[test]
fn test_room_display_name_audit() {
    let room = Room::Audit("a1".to_string());
    assert_eq!(room.display_name(), "audit:a1");
}

#[test]
fn test_room_display_name_user_never_exposes_user_id() {
    let room = Room::User("secret_user_id".to_string());
    let name = room.display_name();
    assert!(!name.contains("secret_user_id"));
    assert_eq!(name, "me");
}
