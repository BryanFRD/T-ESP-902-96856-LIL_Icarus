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
