use crate::utils::event_encryption::{decrypt_json, encrypt_json};
use serde_json::json;

#[test]
fn test_encrypt_decrypt_json() {
    let key = [0u8; 32];
    let json = json!({"password": "secret123", "email": "test@example.com"});

    let (ciphertext, nonce) = encrypt_json(&key, &json).unwrap();
    let decrypted = decrypt_json(&key, &ciphertext, &nonce).unwrap();

    assert_eq!(json, decrypted);
}

#[test]
fn test_different_nonces_produce_different_ciphertexts() {
    let key = [0u8; 32];
    let json = json!({"data": "same content"});

    let (ciphertext1, _) = encrypt_json(&key, &json).unwrap();
    let (ciphertext2, _) = encrypt_json(&key, &json).unwrap();

    assert_ne!(ciphertext1, ciphertext2);
}

#[test]
fn test_wrong_nonce_fails_decryption() {
    let key = [0u8; 32];
    let json = json!({"secret": "data"});

    let (ciphertext, _) = encrypt_json(&key, &json).unwrap();
    let wrong_nonce = vec![0u8; 12];

    let result = decrypt_json(&key, &ciphertext, &wrong_nonce);
    assert!(result.is_err());
}
