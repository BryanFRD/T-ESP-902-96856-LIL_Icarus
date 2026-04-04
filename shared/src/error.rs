use std::io;
use thiserror::Error;

use crate::{encryption::EncryptionError, packets::PacketError};

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Unexpected packet")]
    UnexpectedPacket,
    #[error("Error while handling packet: {0}")]
    PacketError(#[from] PacketError),
    #[error("Error while encrypting or decrypting: {0}")]
    CryptError(#[from] EncryptionError),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Error while converting data")]
    ConvertError,
    #[error("Encryption token didn't match, got {got}, expected {expected}")]
    TokenDontMatch { expected: u64, got: u64 },
    #[error("Failed to lock mutex")]
    LockError,
    #[error("Stream {0} not found")]
    StreamNotFound(u32),
    #[error("Stream {0} already exists")]
    StreamAlreadyExists(u32),
    #[error("Stream {0} closed")]
    StreamClosed(u32),
    #[error("Failed to send on channel")]
    ChannelSendError,
    #[error("Failed to receive on channel")]
    ChannelReceiveError,
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_unexpected_packet() {
        let err = NetworkError::UnexpectedPacket;
        assert_eq!(format!("{err}"), "Unexpected packet");
    }

    #[test]
    fn display_convert_error() {
        let err = NetworkError::ConvertError;
        assert_eq!(format!("{err}"), "Error while converting data");
    }

    #[test]
    fn display_token_dont_match() {
        let err = NetworkError::TokenDontMatch { expected: 42, got: 99 };
        assert!(format!("{err}").contains("42"));
        assert!(format!("{err}").contains("99"));
    }

    #[test]
    fn display_stream_not_found() {
        let err = NetworkError::StreamNotFound(5);
        assert!(format!("{err}").contains('5'));
    }

    #[test]
    fn display_stream_already_exists() {
        let err = NetworkError::StreamAlreadyExists(3);
        assert!(format!("{err}").contains('3'));
    }

    #[test]
    fn display_stream_closed() {
        let err = NetworkError::StreamClosed(7);
        assert!(format!("{err}").contains('7'));
    }

    #[test]
    fn display_lock_error() {
        let err = NetworkError::LockError;
        assert_eq!(format!("{err}"), "Failed to lock mutex");
    }

    #[test]
    fn display_channel_errors() {
        assert_eq!(format!("{}", NetworkError::ChannelSendError), "Failed to send on channel");
        assert_eq!(format!("{}", NetworkError::ChannelReceiveError), "Failed to receive on channel");
    }

    #[test]
    fn display_authentication_failed() {
        let err = NetworkError::AuthenticationFailed("bad token".to_string());
        assert!(format!("{err}").contains("bad token"));
    }
}
