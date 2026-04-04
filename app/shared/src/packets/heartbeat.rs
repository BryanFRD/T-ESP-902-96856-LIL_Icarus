use crate::Packet;
use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x07)]
pub struct Heartbeat {
    pub alive: bool,
}

impl Heartbeat {
    #[must_use]
    pub fn new() -> Self {
        Heartbeat { alive: true }
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat { alive: true }
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
        let hb = Heartbeat::new();
        let bytes = hb.serialize().unwrap();
        // serialize() prepends packet code, deserialize() expects raw data
        let decoded = Heartbeat::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.alive);
    }

    #[test]
    fn packet_code_is_correct() {
        assert_eq!(Heartbeat::packet_code(), 0x07);
    }
}
