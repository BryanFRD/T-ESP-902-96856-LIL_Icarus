use crate::Packet;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0A)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub cpu_name: String,
    pub cpu_count: u32,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub uptime_secs: u64,
}
