use bincode::{Decode, Encode};
use derive::Packet;

/// Packet sent to open a new stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x03)]
pub struct StreamOpen {
    pub stream_id: u32,
}

/// Packet sent to close an existing stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x04)]
pub struct StreamClose {
    pub stream_id: u32,
}

/// Packet containing data for a specific stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x05)]
pub struct StreamData {
    pub stream_id: u32,
    pub data: Vec<u8>,
}

/// Packet indicating an error on a specific stream
#[derive(Debug, Encode, Decode, Packet)]
#[packet(code = 0x06)]
pub struct StreamError {
    pub stream_id: u32,
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn stream_open_roundtrip() {
        let pkt = StreamOpen { stream_id: 42 };
        let bytes = pkt.serialize().unwrap();
        let decoded = StreamOpen::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.stream_id, 42);
    }

    #[test]
    fn stream_close_roundtrip() {
        let pkt = StreamClose { stream_id: 99 };
        let bytes = pkt.serialize().unwrap();
        let decoded = StreamClose::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.stream_id, 99);
    }

    #[test]
    fn stream_data_roundtrip() {
        let pkt = StreamData { stream_id: 5, data: vec![0xDE, 0xAD, 0xBE, 0xEF] };
        let bytes = pkt.serialize().unwrap();
        let decoded = StreamData::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.stream_id, 5);
        assert_eq!(decoded.data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn stream_data_empty() {
        let pkt = StreamData { stream_id: 1, data: vec![] };
        let bytes = pkt.serialize().unwrap();
        let decoded = StreamData::deserialize(&bytes[1..]).unwrap();
        assert!(decoded.data.is_empty());
    }

    #[test]
    fn stream_error_roundtrip() {
        let pkt = StreamError { stream_id: 3, error: "broken".to_string() };
        let bytes = pkt.serialize().unwrap();
        let decoded = StreamError::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.stream_id, 3);
        assert_eq!(decoded.error, "broken");
    }

    #[test]
    fn packet_codes_are_correct() {
        assert_eq!(StreamOpen::packet_code(), 0x03);
        assert_eq!(StreamClose::packet_code(), 0x04);
        assert_eq!(StreamData::packet_code(), 0x05);
        assert_eq!(StreamError::packet_code(), 0x06);
    }
}
