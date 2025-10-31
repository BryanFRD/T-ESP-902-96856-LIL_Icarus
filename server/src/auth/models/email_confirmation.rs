use base64::Engine;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shared::encryption::{decrypt, encrypt, EncryptionError};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfirmationToken {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum ConfirmationTokenError {
    #[error("Failed to serialize token")]
    SerializationError(#[from] serde_json::Error),
    #[error("Token encryption failed: {0}")]
    EncryptionError(#[from] EncryptionError),
    #[error("Token has expired")]
    TokenExpired,
    #[error("Invalid base64 encoding")]
    Base64Error(#[from] base64::DecodeError),
}

impl EmailConfirmationToken {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
    ) -> Self {
        let expires_at = Utc::now() + Duration::minutes(10);
        Self {
            username,
            first_name,
            last_name,
            email,
            password_hash,
            expires_at,
        }
    }

    pub fn encrypt(&self, key: &[u8]) -> Result<String, ConfirmationTokenError> {
        let aes_key = Self::derive_aes_key(key);
        let json = serde_json::to_string(self)?;
        let (ciphertext, nonce) = encrypt(&aes_key, json.as_bytes())?;
        let mut combined = nonce;
        combined.extend_from_slice(&ciphertext);
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&combined);
        Ok(encoded)
    }

    pub fn decrypt(token: &str, key: &[u8]) -> Result<Self, ConfirmationTokenError> {
        let aes_key = Self::derive_aes_key(key);
        let combined = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(token)?;
        if combined.len() < 12 {
            return Err(ConfirmationTokenError::EncryptionError(
                EncryptionError::FailedToDecrypt("Token too short".to_string()),
            ));
        }
        let (nonce, ciphertext) = combined.split_at(12);
        let plaintext = decrypt(&aes_key, nonce, ciphertext)?;
        let token: Self = serde_json::from_slice(&plaintext)?;
        if token.is_expired() {
            return Err(ConfirmationTokenError::TokenExpired);
        }
        Ok(token)
    }

    fn derive_aes_key(key: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let result = hasher.finalize();
        let mut aes_key = [0u8; 32];
        aes_key.copy_from_slice(&result);
        aes_key
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
