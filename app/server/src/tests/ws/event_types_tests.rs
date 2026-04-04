use crate::api::ws::events::types::{
    AgentKeyloggerPayload, AgentOutputPayload, AgentStatusPayload, AgentStreamClosedPayload,
    InvitationReceivedPayload, UserNotificationPayload,
};

#[test]
fn test_user_notification_payload_serialization() {
    let payload = UserNotificationPayload {
        title: "New message".to_string(),
        message: "You have a new message".to_string(),
        notification_type: "info".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["title"], "New message");
    assert_eq!(json["message"], "You have a new message");
    assert_eq!(json["notification_type"], "info");
}

#[test]
fn test_invitation_received_payload_with_team() {
    let payload = InvitationReceivedPayload {
        invitation_id: "inv1".to_string(),
        team_name: Some("Alpha Team".to_string()),
        audit_name: None,
        invited_by: "admin".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["invitation_id"], "inv1");
    assert_eq!(json["team_name"], "Alpha Team");
    assert!(json.get("audit_name").is_none());
    assert_eq!(json["invited_by"], "admin");
}

#[test]
fn test_invitation_received_payload_with_audit() {
    let payload = InvitationReceivedPayload {
        invitation_id: "inv2".to_string(),
        team_name: None,
        audit_name: Some("Security Audit Q1".to_string()),
        invited_by: "pentester".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert!(json.get("team_name").is_none());
    assert_eq!(json["audit_name"], "Security Audit Q1");
}

#[test]
fn test_agent_output_payload_serialization() {
    let payload = AgentOutputPayload {
        agent_id: "agent1".to_string(),
        stream_id: 42,
        data: "SGVsbG8=".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["agent_id"], "agent1");
    assert_eq!(json["stream_id"], 42);
    assert_eq!(json["data"], "SGVsbG8=");
}

#[test]
fn test_agent_status_payload_online() {
    let payload = AgentStatusPayload {
        agent_id: "agent1".to_string(),
        online: true,
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["agent_id"], "agent1");
    assert_eq!(json["online"], true);
}

#[test]
fn test_agent_status_payload_offline() {
    let payload = AgentStatusPayload {
        agent_id: "agent1".to_string(),
        online: false,
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["online"], false);
}

#[test]
fn test_agent_stream_closed_payload_serialization() {
    let payload = AgentStreamClosedPayload {
        agent_id: "agent1".to_string(),
        stream_id: 7,
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["agent_id"], "agent1");
    assert_eq!(json["stream_id"], 7);
}

#[test]
fn test_agent_keylogger_payload_serialization() {
    let payload = AgentKeyloggerPayload {
        agent_id: "agent1".to_string(),
        data: "a2V5cw==".to_string(),
    };
    let json = serde_json::to_value(&payload).unwrap();
    assert_eq!(json["agent_id"], "agent1");
    assert_eq!(json["data"], "a2V5cw==");
}
