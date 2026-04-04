use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::Rng;
use thiserror::Error;

/// Encrypts plaintext using AES-256-GCM.
///
/// # Errors
/// Returns an error if encryption fails.
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EncryptionError> {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    let nonce: [u8; 12] = rand::rng().random();
    let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), plaintext) {
        Ok(ct) => ct,
        Err(e) => return Err(EncryptionError::FailedToEncrypt(e.to_string())),
    };
    Ok((ciphertext, nonce.to_vec()))
}

/// Decrypts ciphertext using AES-256-GCM.
///
/// # Errors
/// Returns an error if decryption fails.
pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    match cipher.decrypt(Nonce::from_slice(nonce), ciphertext) {
        Ok(decrypted) => Ok(decrypted),
        Err(e) => Err(EncryptionError::FailedToDecrypt(e.to_string())),
    }
}

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Failed to encrypt: {0}")]
    FailedToEncrypt(String),
    #[error("Failed to decrypt: {0}")]
    FailedToDecrypt(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [0xAB; 32]
    }

    #[test]
    fn encrypt_then_decrypt_roundtrip() {
        let key = test_key();
        let plaintext = b"hello world";
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn decrypt_with_wrong_key_fails() {
        let key = test_key();
        let wrong_key = [0xCD; 32];
        let plaintext = b"secret data";
        let (ciphertext, nonce) = encrypt(&key, plaintext).unwrap();
        assert!(decrypt(&wrong_key, &nonce, &ciphertext).is_err());
    }

    #[test]
    fn decrypt_with_wrong_nonce_fails() {
        let key = test_key();
        let plaintext = b"secret data";
        let (ciphertext, _nonce) = encrypt(&key, plaintext).unwrap();
        let wrong_nonce = vec![0x00; 12];
        assert!(decrypt(&key, &wrong_nonce, &ciphertext).is_err());
    }

    #[test]
    fn encrypt_produces_different_nonces() {
        let key = test_key();
        let plaintext = b"same input";
        let (_ct1, nonce1) = encrypt(&key, plaintext).unwrap();
        let (_ct2, nonce2) = encrypt(&key, plaintext).unwrap();
        assert_ne!(nonce1, nonce2);
    }

    #[test]
    fn encrypt_decrypt_empty_plaintext() {
        let key = test_key();
        let (ciphertext, nonce) = encrypt(&key, b"").unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn ciphertext_differs_from_plaintext() {
        let key = test_key();
        let plaintext = b"hello world";
        let (ciphertext, _) = encrypt(&key, plaintext).unwrap();
        assert_ne!(ciphertext, plaintext);
    }

    #[test]
    fn decrypt_tampered_ciphertext_fails() {
        let key = test_key();
        let (mut ciphertext, nonce) = encrypt(&key, b"data").unwrap();
        if let Some(byte) = ciphertext.first_mut() {
            *byte ^= 0xFF;
        }
        assert!(decrypt(&key, &nonce, &ciphertext).is_err());
    }

    #[test]
    fn encryption_error_display() {
        let err = EncryptionError::FailedToEncrypt("test".to_string());
        assert_eq!(format!("{err}"), "Failed to encrypt: test");
        let err = EncryptionError::FailedToDecrypt("test".to_string());
        assert_eq!(format!("{err}"), "Failed to decrypt: test");
    }
}
