use crate::Packet;
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x07)]
pub struct Heartbeat {
    pub alive: bool,
    /// Timestamp in milliseconds (epoch) set by the sender.
    /// Used to measure round-trip time: agent sets it on send,
    /// server echoes it back unchanged.
    pub sent_at_ms: u64,
    /// Round-trip latency in ms measured by the agent from the
    /// previous heartbeat echo. 0 means no measurement yet.
    pub latency_ms: u64,
}

impl Heartbeat {
    #[must_use]
    pub fn new() -> Self {
        Heartbeat {
            alive: true,
            sent_at_ms: 0,
            latency_ms: 0,
        }
    }

    #[must_use]
    pub fn with_timing(sent_at_ms: u64, latency_ms: u64) -> Self {
        Heartbeat {
            alive: true,
            sent_at_ms,
            latency_ms,
        }
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn new_creates_alive_heartbeat() {
        let hb = Heartbeat::new();
        assert!(hb.alive);
    }

    #[test]
    fn default_creates_alive_heartbeat() {
        let hb = Heartbeat::default();
        assert!(hb.alive);
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let hb = Heartbeat::with_timing(1_000_000, 42);
        let bytes = hb.serialize().unwrap();
        let decoded = Heartbeat::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.alive);
        assert_eq!(decoded.sent_at_ms, 1_000_000);
        assert_eq!(decoded.latency_ms, 42);
    }

    #[test]
    fn packet_code_is_correct() {
        assert_eq!(Heartbeat::packet_code(), 0x07);
    }
}
