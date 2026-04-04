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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn serialize_deserialize_roundtrip() {
        let info = SystemInfo {
            hostname: "server-01".to_string(),
            os_name: "Linux".to_string(),
            os_version: "6.1".to_string(),
            cpu_name: "Intel i9".to_string(),
            cpu_count: 16,
            total_memory_bytes: 32_000_000_000,
            used_memory_bytes: 16_000_000_000,
            uptime_secs: 86400,
        };
        let bytes = info.serialize().unwrap();
        let decoded = SystemInfo::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.hostname, "server-01");
        assert_eq!(decoded.os_name, "Linux");
        assert_eq!(decoded.cpu_count, 16);
        assert_eq!(decoded.total_memory_bytes, 32_000_000_000);
        assert_eq!(decoded.uptime_secs, 86400);
    }

    #[test]
    fn packet_code_is_correct() {
        assert_eq!(SystemInfo::packet_code(), 0x0A);
    }
}
