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
}
