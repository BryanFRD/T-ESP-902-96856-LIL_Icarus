use crate::api::ws::rooms::handlers::{AgentCommandPayload, AgentShutdownPayload};

#[test]
fn test_agent_shutdown_payload_deserialization() {
    let json = r#"{"audit_id":"a1","agent_id":"ag1","reason":"maintenance"}"#;
    let payload: AgentShutdownPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.audit_id, "a1");
    assert_eq!(payload.agent_id, "ag1");
    assert_eq!(payload.reason.as_deref(), Some("maintenance"));
}

#[test]
fn test_agent_shutdown_payload_without_reason() {
    let json = r#"{"audit_id":"a1","agent_id":"ag1"}"#;
    let payload: AgentShutdownPayload = serde_json::from_str(json).unwrap();
    assert!(payload.reason.is_none());
}

#[test]
fn test_agent_command_payload_deserialization() {
    let json = r#"{"audit_id":"a1","agent_id":"ag1","command":"whoami"}"#;
    let payload: AgentCommandPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.audit_id, "a1");
    assert_eq!(payload.agent_id, "ag1");
    assert_eq!(payload.command, "whoami");
}
