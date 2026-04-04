use bincode::{Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x08)]
pub struct AgentAuth {
    pub audit_id: String,
    pub token: String,
}

impl AgentAuth {
    #[must_use]
    pub fn new(audit_id: String, token: String) -> Self {
        Self { audit_id, token }
    }
}

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x09)]
pub struct AgentAuthResponse {
    pub success: bool,
    pub message: String,
}

impl AgentAuthResponse {
    #[must_use]
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn agent_auth_roundtrip() {
        let auth = AgentAuth::new("audit-123".to_string(), "token-xyz".to_string());
        let bytes = auth.serialize().unwrap();
        let decoded = AgentAuth::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.audit_id, "audit-123");
        assert_eq!(decoded.token, "token-xyz");
    }

    #[test]
    fn agent_auth_response_success() {
        let resp = AgentAuthResponse::new(true, "welcome".to_string());
        let bytes = resp.serialize().unwrap();
        let decoded = AgentAuthResponse::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.success);
        assert_eq!(decoded.message, "welcome");
    }

    #[test]
    fn agent_auth_response_failure() {
        let resp = AgentAuthResponse::new(false, "denied".to_string());
        let bytes = resp.serialize().unwrap();
        let decoded = AgentAuthResponse::deserialize(&bytes[1..]).unwrap();
        assert!(!decoded.success);
        assert_eq!(decoded.message, "denied");
    }

    #[test]
    fn packet_codes_are_correct() {
        assert_eq!(AgentAuth::packet_code(), 0x08);
        assert_eq!(AgentAuthResponse::packet_code(), 0x09);
    }
}
