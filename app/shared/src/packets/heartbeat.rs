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
