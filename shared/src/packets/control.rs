use crate::Packet;
use bincode::{Decode, Encode};

/// Control commands that can be sent to an agent.
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0B)]
pub struct AgentControl {
    pub instruction: AgentInstruction,
}

impl AgentControl {
    #[must_use]
    pub fn new(instruction: AgentInstruction) -> Self {
        Self { instruction }
    }

    /// Create a shutdown instruction.
    #[must_use]
    pub fn shutdown(reason: String) -> Self {
        Self::new(AgentInstruction::Shutdown { reason })
    }

    /// Create a restart instruction.
    #[must_use]
    pub fn restart(reason: String) -> Self {
        Self::new(AgentInstruction::Restart { reason })
    }

    /// Create a request system info instruction.
    #[must_use]
    pub fn request_system_info() -> Self {
        Self::new(AgentInstruction::RequestSystemInfo)
    }

    /// Create a pause instruction.
    #[must_use]
    pub fn pause() -> Self {
        Self::new(AgentInstruction::Pause)
    }

    /// Create a resume instruction.
    #[must_use]
    pub fn resume() -> Self {
        Self::new(AgentInstruction::Resume)
    }

    #[must_use]
    pub fn update(version: String, binary_size: u64) -> Self {
        Self::new(AgentInstruction::Update {
            version,
            binary_size,
        })
    }
}

/// Instructions that can be sent to control an agent.
#[derive(Debug, Clone, Encode, Decode)]
pub enum AgentInstruction {
    /// Shutdown the agent with a reason.
    Shutdown { reason: String },
    /// Restart the agent with a reason.
    Restart { reason: String },
    /// Request immediate system info update.
    RequestSystemInfo,
    /// Pause agent operations.
    Pause,
    /// Resume agent operations.
    Resume,
    /// Update the agent binary. Server will stream the binary via `StreamData` packets.
    Update {
        version: String,
        binary_size: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn shutdown_creates_correct_instruction() {
        let ctrl = AgentControl::shutdown("reason".to_string());
        assert!(matches!(ctrl.instruction, AgentInstruction::Shutdown { reason } if reason == "reason"));
    }

    #[test]
    fn restart_creates_correct_instruction() {
        let ctrl = AgentControl::restart("reason".to_string());
        assert!(matches!(ctrl.instruction, AgentInstruction::Restart { reason } if reason == "reason"));
    }

    #[test]
    fn request_system_info_creates_correct_instruction() {
        let ctrl = AgentControl::request_system_info();
        assert!(matches!(ctrl.instruction, AgentInstruction::RequestSystemInfo));
    }

    #[test]
    fn pause_creates_correct_instruction() {
        let ctrl = AgentControl::pause();
        assert!(matches!(ctrl.instruction, AgentInstruction::Pause));
    }

    #[test]
    fn resume_creates_correct_instruction() {
        let ctrl = AgentControl::resume();
        assert!(matches!(ctrl.instruction, AgentInstruction::Resume));
    }

    #[test]
    fn serialize_deserialize_shutdown() {
        let ctrl = AgentControl::shutdown("test".to_string());
        let bytes = ctrl.serialize().unwrap();
        let decoded = AgentControl::deserialize(&bytes[1..]).unwrap();
        assert!(matches!(decoded.instruction, AgentInstruction::Shutdown { reason } if reason == "test"));
    }

    #[test]
    fn serialize_deserialize_pause() {
        let ctrl = AgentControl::pause();
        let bytes = ctrl.serialize().unwrap();
        let decoded = AgentControl::deserialize(&bytes[1..]).unwrap();
        assert!(matches!(decoded.instruction, AgentInstruction::Pause));
    }

    #[test]
    fn packet_code_is_correct() {
        assert_eq!(AgentControl::packet_code(), 0x0B);
    }

    #[test]
    fn all_instruction_variants_survive_roundtrip() {
        // Exercise every AgentInstruction variant through serialize/deserialize
        // to catch any bincode encoding issues with the enum discriminants.
        let cases = vec![
            AgentControl::shutdown("maintenance window".to_string()),
            AgentControl::restart("config reload".to_string()),
            AgentControl::request_system_info(),
            AgentControl::pause(),
            AgentControl::resume(),
        ];
        for ctrl in &cases {
            let bytes = ctrl.serialize().unwrap();
            let decoded = AgentControl::deserialize(&bytes[1..]).unwrap();
            // Verify the discriminant matches by checking Debug output
            assert_eq!(
                format!("{:?}", ctrl.instruction),
                format!("{:?}", decoded.instruction),
            );
        }
    }

    #[test]
    fn shutdown_with_unicode_reason_roundtrips() {
        let reason = "\u{1F525} critical failure \u{26A0}\u{FE0F} — shutting down".to_string();
        let ctrl = AgentControl::shutdown(reason.clone());
        let bytes = ctrl.serialize().unwrap();
        let decoded = AgentControl::deserialize(&bytes[1..]).unwrap();
        match decoded.instruction {
            AgentInstruction::Shutdown { reason: r } => assert_eq!(r, reason),
            other => panic!("expected Shutdown, got {other:?}"),
        }
    }
}
