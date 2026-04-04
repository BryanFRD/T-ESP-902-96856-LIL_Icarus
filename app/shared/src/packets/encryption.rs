use bincode::{self, Decode, Encode};
use derive::Packet;

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x01)]
pub struct EncryptionRequest {
    pub key: [u8; 32],
    pub verify_token: u64,
}

impl EncryptionRequest {
    pub const PACKET_SIZE: usize = 42;

    #[must_use]
    pub fn new(key: [u8; 32], verify_token: u64) -> Self {
        EncryptionRequest { key, verify_token }
    }
}

#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x02)]
pub struct EncryptionResponse {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
    pub verify_token: [u8; 24],
}

impl EncryptionResponse {
    pub const PACKET_SIZE: usize = 69;

    #[must_use]
    pub fn new(key: [u8; 32], nonce: [u8; 12], verify_token: [u8; 24]) -> Self {
        EncryptionResponse {
            key,
            nonce,
            verify_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn encryption_request_roundtrip() {
        let key = [0xAA; 32];
        let req = EncryptionRequest::new(key, 12345);
        let bytes = req.serialize().unwrap();
        let decoded = EncryptionRequest::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.key, key);
        assert_eq!(decoded.verify_token, 12345);
    }

    #[test]
    fn encryption_response_roundtrip() {
        let key = [0xBB; 32];
        let nonce = [0xCC; 12];
        let token = [0xDD; 24];
        let resp = EncryptionResponse::new(key, nonce, token);
        let bytes = resp.serialize().unwrap();
        let decoded = EncryptionResponse::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.key, key);
        assert_eq!(decoded.nonce, nonce);
        assert_eq!(decoded.verify_token, token);
    }

    #[test]
    fn packet_codes_are_correct() {
        assert_eq!(EncryptionRequest::packet_code(), 0x01);
        assert_eq!(EncryptionResponse::packet_code(), 0x02);
    }

    #[test]
    fn packet_sizes_are_correct() {
        assert_eq!(EncryptionRequest::PACKET_SIZE, 42);
        assert_eq!(EncryptionResponse::PACKET_SIZE, 69);
    }
}
