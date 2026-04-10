use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x08)]
pub struct AgentAuth {
    pub audit_id: String,
    pub token: String,
    pub hostname: String,
}

impl AgentAuth {
    #[must_use]
    pub fn new(audit_id: String, token: String, hostname: String) -> Self {
        Self {
            audit_id,
            token,
            hostname,
        }
    }
}

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x09)]
pub struct AgentAuthResponse {
    pub success: bool,
    pub message: String,
    pub agent_token: String,
    pub agent_id: String,
}

impl AgentAuthResponse {
    #[must_use]
    pub fn new(success: bool, message: String) -> Self {
        Self {
            success,
            message,
            agent_token: String::new(),
            agent_id: String::new(),
        }
    }

    #[must_use]
    pub fn with_agent(
        success: bool,
        message: String,
        agent_token: String,
        agent_id: String,
    ) -> Self {
        Self {
            success,
            message,
            agent_token,
            agent_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn agent_auth_roundtrip() {
        let auth = AgentAuth::new(
            "audit-123".to_string(),
            "token-xyz".to_string(),
            "PC-JOHN".to_string(),
        );
        let bytes = auth.serialize().unwrap();
        let decoded = AgentAuth::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.audit_id, "audit-123");
        assert_eq!(decoded.token, "token-xyz");
        assert_eq!(decoded.hostname, "PC-JOHN");
    }

    #[test]
    fn agent_auth_response_success() {
        let resp = AgentAuthResponse::with_agent(
            true,
            "welcome".to_string(),
            "agent-token-123".to_string(),
            "agent-id-456".to_string(),
        );
        let bytes = resp.serialize().unwrap();
        let decoded = AgentAuthResponse::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.success);
        assert_eq!(decoded.message, "welcome");
        assert_eq!(decoded.agent_token, "agent-token-123");
        assert_eq!(decoded.agent_id, "agent-id-456");
    }

    #[test]
    fn agent_auth_response_failure() {
        let resp = AgentAuthResponse::new(false, "denied".to_string());
        let bytes = resp.serialize().unwrap();
        let decoded = AgentAuthResponse::deserialize(&bytes[1..]).unwrap();
        assert!(!decoded.success);
        assert_eq!(decoded.message, "denied");
        assert!(decoded.agent_token.is_empty());
        assert!(decoded.agent_id.is_empty());
    }

    #[test]
    fn packet_codes_are_correct() {
        assert_eq!(AgentAuth::packet_code(), 0x08);
        assert_eq!(AgentAuthResponse::packet_code(), 0x09);
    }
}
