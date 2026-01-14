use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value as JsonValue;
use shared::encryption::{EncryptionError, decrypt, encrypt};

pub fn encrypt_json(key: &[u8], json: &JsonValue) -> Result<(String, Vec<u8>), EncryptionError> {
    let plaintext =
        serde_json::to_vec(json).map_err(|e| EncryptionError::FailedToEncrypt(e.to_string()))?;

    let (ciphertext, nonce) = encrypt(key, &plaintext)?;
    let ciphertext_b64 = STANDARD.encode(&ciphertext);

    Ok((ciphertext_b64, nonce))
}

pub fn decrypt_json(
    key: &[u8],
    ciphertext_b64: &str,
    nonce: &[u8],
) -> Result<JsonValue, EncryptionError> {
    let ciphertext = STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| EncryptionError::FailedToDecrypt(e.to_string()))?;

    let plaintext = decrypt(key, nonce, &ciphertext)?;

    serde_json::from_slice(&plaintext).map_err(|e| EncryptionError::FailedToDecrypt(e.to_string()))
}
