#[cfg(test)]
mod tests {
    use crate::api::agents::{
        AuditAgent, CreateAgentPayload, KeyloggerAction, KeyloggerRequestPayload,
        KeyloggerResponse, ShellRequestPayload, ShellStreamResponse, ShellType,
        SystemInfoResponse,
    };
    use crate::api::agents::error::AgentError;

    // AuditAgent tests
    #[test]
    fn test_audit_agent_new() {
        let agent = AuditAgent::new("audit1".into(), "my-agent".into(), "hash123".into());
        assert_eq!(agent.audit_id, "audit1");
        assert_eq!(agent.name, "my-agent");
        assert_eq!(agent.token_hash, "hash123");
        assert!(!agent.is_revoked);
        assert!(agent.last_connected_at.is_none());
        assert!(!agent.id.is_empty());
    }

    #[test]
    fn test_audit_agent_unique_ids() {
        let a1 = AuditAgent::new("a".into(), "n".into(), "h".into());
        let a2 = AuditAgent::new("a".into(), "n".into(), "h".into());
        assert_ne!(a1.id, a2.id);
    }

    // ShellType serde
    #[test]
    fn test_shell_type_serde_roundtrip() {
        for (variant, expected_json) in [
            (ShellType::Auto, "\"auto\""),
            (ShellType::Bash, "\"bash\""),
            (ShellType::Sh, "\"sh\""),
            (ShellType::Cmd, "\"cmd\""),
            (ShellType::PowerShell, "\"powershell\""),
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected_json);
            let deserialized: ShellType = serde_json::from_str(&json).unwrap();
            assert_eq!(
                format!("{deserialized:?}"),
                format!("{variant:?}")
            );
        }
    }

    // ShellType conversion to shared::packets::ShellType
    #[test]
    fn test_shell_type_to_shared() {
        let cases = [
            ShellType::Auto,
            ShellType::Bash,
            ShellType::Sh,
            ShellType::Cmd,
            ShellType::PowerShell,
        ];
        for st in cases {
            let shared_st: shared::packets::ShellType = st.into();
            // Just verify conversion doesn't panic
            let _ = format!("{shared_st:?}");
        }
    }

    // KeyloggerAction serde
    #[test]
    fn test_keylogger_action_serde() {
        let start_json = serde_json::to_string(&KeyloggerAction::Start).unwrap();
        assert_eq!(start_json, "\"start\"");
        let stop_json = serde_json::to_string(&KeyloggerAction::Stop).unwrap();
        assert_eq!(stop_json, "\"stop\"");

        let deserialized: KeyloggerAction = serde_json::from_str("\"start\"").unwrap();
        assert!(matches!(deserialized, KeyloggerAction::Start));
        let deserialized: KeyloggerAction = serde_json::from_str("\"stop\"").unwrap();
        assert!(matches!(deserialized, KeyloggerAction::Stop));
    }

    // CreateAgentPayload serde
    #[test]
    fn test_create_agent_payload_serde() {
        let json = r#"{"name":"agent-01"}"#;
        let payload: CreateAgentPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "agent-01");
    }

    // ShellRequestPayload serde
    #[test]
    fn test_shell_request_payload_serde() {
        let json = r#"{"shellType":"bash"}"#;
        let payload: ShellRequestPayload = serde_json::from_str(json).unwrap();
        assert!(matches!(payload.shell_type, ShellType::Bash));
    }

    // KeyloggerRequestPayload serde
    #[test]
    fn test_keylogger_request_payload_serde() {
        let json = r#"{"action":"start"}"#;
        let payload: KeyloggerRequestPayload = serde_json::from_str(json).unwrap();
        assert!(matches!(payload.action, KeyloggerAction::Start));
    }

    // SystemInfoResponse from shared
    #[test]
    fn test_system_info_response_from_shared() {
        let info = shared::packets::SystemInfo {
            hostname: "host1".into(),
            os_name: "Linux".into(),
            os_version: "6.0".into(),
            cpu_name: "AMD".into(),
            cpu_count: 8,
            total_memory_bytes: 16_000_000_000,
            used_memory_bytes: 8_000_000_000,
            uptime_secs: 3600,
        };
        let resp = SystemInfoResponse::from(info);
        assert_eq!(resp.hostname, "host1");
        assert_eq!(resp.os_name, "Linux");
        assert_eq!(resp.os_version, "6.0");
        assert_eq!(resp.cpu_name, "AMD");
        assert_eq!(resp.cpu_count, 8);
        assert_eq!(resp.total_memory_bytes, 16_000_000_000);
        assert_eq!(resp.used_memory_bytes, 8_000_000_000);
        assert_eq!(resp.uptime_secs, 3600);
    }

    // ShellStreamResponse serialization
    #[test]
    fn test_shell_stream_response_serialize() {
        let resp = ShellStreamResponse {
            stream_id: 42,
            message: "Shell session requested".into(),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["streamId"], 42);
        assert_eq!(json["message"], "Shell session requested");
    }

    // KeyloggerResponse serialization
    #[test]
    fn test_keylogger_response_serialize() {
        let resp = KeyloggerResponse {
            message: "Keylogger started".into(),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["message"], "Keylogger started");
    }

    // AgentError Display
    #[test]
    fn test_agent_error_display() {
        assert_eq!(AgentError::NotFound.to_string(), "Agent not found");
        assert_eq!(
            AgentError::NameAlreadyExists.to_string(),
            "An agent with this name already exists for this audit"
        );
        assert_eq!(
            AgentError::MustBeOwnerOrAdmin.to_string(),
            "Must be team owner or admin"
        );
        assert_eq!(AgentError::Revoked.to_string(), "Agent has been revoked");
        assert_eq!(AgentError::InvalidToken.to_string(), "Invalid agent token");
        assert_eq!(AgentError::AgentNotOnline.to_string(), "Agent is not online");
        assert_eq!(
            AgentError::ShellRequestFailed("timeout".into()).to_string(),
            "Failed to send shell request: timeout"
        );
        assert_eq!(
            AgentError::KeyloggerRequestFailed("err".into()).to_string(),
            "Failed to send keylogger request: err"
        );
    }
}
