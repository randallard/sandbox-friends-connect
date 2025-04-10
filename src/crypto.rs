use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

// Structure to represent encrypted data
#[derive(Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: String,  // Base64 encoded encrypted data
    pub iv: String,          // Base64 encoded initialization vector
    pub tag: String,         // Base64 encoded authentication tag
}

// Error type for crypto operations
#[derive(Debug, Clone)]
pub enum CryptoError {
    EncryptionError(String),
    DecryptionError(String),
    EncodingError(String),
    KeyError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            CryptoError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            CryptoError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            CryptoError::KeyError(msg) => write!(f, "Key error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

// Key derivation from environment or fixed for testing
fn get_encryption_key() -> Result<Key<Aes256Gcm>, CryptoError> {
    // In production, you'd want to derive this from environment or secure storage
    // For testing purposes, we're using a fixed key (NEVER DO THIS IN PRODUCTION)
    let key_bytes = [
        0x42, 0x64, 0x2c, 0x0f, 0x1c, 0x51, 0x9a, 0xeb,
        0x85, 0x33, 0xfd, 0x75, 0x2a, 0x1f, 0xe9, 0x03,
        0x54, 0x12, 0x9c, 0xb5, 0x7d, 0x29, 0x1a, 0x3c, 
        0x6e, 0x5e, 0x02, 0x9b, 0xd3, 0xf6, 0xa1, 0xc7
    ];
    
    Ok(Key::<Aes256Gcm>::from_slice(&key_bytes).clone())
}

// Encrypt data and return as JSON string
pub fn encrypt_data(data: &str) -> Result<String, CryptoError> {
    let key = get_encryption_key()?;
    let cipher = Aes256Gcm::new(&key);
    
    // Generate random IV (nonce)
    let iv = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Encrypt the data
    let ciphertext = cipher.encrypt(&iv, data.as_bytes().as_ref())
        .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;
    
    // Create the encrypted data structure
    let encrypted = EncryptedData {
        ciphertext: BASE64.encode(&ciphertext),
        iv: BASE64.encode(iv.as_slice()),
        tag: String::new(), // AES-GCM includes the tag in the ciphertext
    };
    
    // Serialize to JSON
    serde_json::to_string(&encrypted)
        .map_err(|e| CryptoError::EncodingError(e.to_string()))
}

// Decrypt data from JSON string
pub fn decrypt_data(encrypted_json: &str) -> Result<String, CryptoError> {
    // Parse the JSON
    let encrypted: EncryptedData = serde_json::from_str(encrypted_json)
        .map_err(|e| CryptoError::EncodingError(format!("Invalid JSON format: {}", e)))?;
    
    // Get the key
    let key = get_encryption_key()?;
    let cipher = Aes256Gcm::new(&key);
    
    // Decode base64 values
    let ciphertext = BASE64.decode(encrypted.ciphertext.as_bytes())
        .map_err(|e| CryptoError::EncodingError(format!("Invalid base64 ciphertext: {}", e)))?;
    
    let iv_bytes = BASE64.decode(encrypted.iv.as_bytes())
        .map_err(|e| CryptoError::EncodingError(format!("Invalid base64 IV: {}", e)))?;
    
    if iv_bytes.len() != 12 {
        return Err(CryptoError::DecryptionError("Invalid IV length".to_string()));
    }
    
    // Create nonce from bytes
    let nonce = Nonce::from_slice(&iv_bytes);
    
    // Decrypt the data
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| CryptoError::DecryptionError(format!("Decryption failed, data may be tampered: {}", e)))?;
    
    // Convert bytes to string
    String::from_utf8(plaintext)
        .map_err(|e| CryptoError::DecryptionError(format!("Invalid UTF-8 in decrypted data: {}", e)))
}

// Verify data integrity without decrypting fully
pub fn verify_data_integrity(encrypted_json: &str) -> Result<bool, CryptoError> {
    // This is a lightweight check that the JSON is valid and has expected fields
    match serde_json::from_str::<EncryptedData>(encrypted_json) {
        Ok(_) => Ok(true),  // Structure is valid
        Err(e) => Err(CryptoError::EncodingError(format!("Invalid encrypted data format: {}", e)))
    }
    // Note: Full integrity verification happens during decryption with AES-GCM
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_encrypt_decrypt_roundtrip() {
        let original_data = r#"{"player_id":"test123","dark_mode":true}"#;
        
        // Encrypt the data
        let encrypted = encrypt_data(original_data).expect("Encryption should succeed");
        
        // Verify it's valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&encrypted);
        assert!(parsed.is_ok(), "Encrypted output should be valid JSON");
        
        // Decrypt the data
        let decrypted = decrypt_data(&encrypted).expect("Decryption should succeed");
        
        // Compare with original
        assert_eq!(decrypted, original_data, "Decrypted data should match original");
    }    
        
    #[wasm_bindgen_test]
    fn test_tampering_detection() {
        let original_data = r#"{"player_id":"tamper_test","dark_mode":false}"#;
        
        // Encrypt the data
        let encrypted = encrypt_data(original_data).expect("Encryption should succeed");
        
        // Parse the encrypted JSON to modify the ciphertext directly
        let mut encrypted_obj: EncryptedData = serde_json::from_str(&encrypted)
            .expect("Should be able to parse our own encrypted JSON");
        
        // Get the original ciphertext and modify it
        let mut modified_ciphertext = encrypted_obj.ciphertext.clone();
        
        // Make sure we're actually changing something by modifying the last character
        // This ensures we're tampering with the actual encrypted data
        if !modified_ciphertext.is_empty() {
            let last_char = modified_ciphertext.chars().last().unwrap();
            let replacement = if last_char == 'A' { 'B' } else { 'A' };
            
            modified_ciphertext.pop();
            modified_ciphertext.push(replacement);
            
            encrypted_obj.ciphertext = modified_ciphertext;
        } else {
            // Fallback in case the ciphertext is empty (shouldn't happen)
            encrypted_obj.ciphertext = "tampered".to_string();
        }
        
        // Serialize back to JSON
        let tampered = serde_json::to_string(&encrypted_obj)
            .expect("Should be able to serialize tampered data");
        
        // Attempt to decrypt tampered data - should fail
        let result = decrypt_data(&tampered);
        assert!(result.is_err(), "Decryption of tampered data should fail");
        
        // Check error message
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("Decryption failed") || error.contains("tampered") || error.contains("Invalid"),
            "Error should indicate tampering: {}", error
        );
    }

    #[wasm_bindgen_test]
    fn test_invalid_json_handling() {
        // Test with completely invalid data
        let result = decrypt_data("not json data");
        assert!(result.is_err(), "Decryption of invalid JSON should fail");
        
        // Test with JSON missing required fields
        let result = decrypt_data(r#"{"some_field": "value"}"#);
        assert!(result.is_err(), "Decryption of JSON with missing fields should fail");
    }
    
    #[wasm_bindgen_test]
    fn test_encryption_produces_different_outputs() {
        let data = r#"{"player_id":"unique_test","dark_mode":true}"#;
        
        // Encrypt the same data twice
        let encrypted1 = encrypt_data(data).expect("First encryption should succeed");
        let encrypted2 = encrypt_data(data).expect("Second encryption should succeed");
        
        // Outputs should be different due to random IV
        assert_ne!(encrypted1, encrypted2, "Encrypting the same data twice should produce different results");
        
        // But both should decrypt to the same original data
        let decrypted1 = decrypt_data(&encrypted1).expect("First decryption should succeed");
        let decrypted2 = decrypt_data(&encrypted2).expect("Second decryption should succeed");
        
        assert_eq!(decrypted1, data, "First decryption should match original");
        assert_eq!(decrypted2, data, "Second decryption should match original");
    }
    
    #[wasm_bindgen_test]
    fn test_data_structure_validation() {
        let data = r#"{"player_id":"integrity_test","dark_mode":true}"#;
        
        // Encrypt valid data
        let encrypted = encrypt_data(data).expect("Encryption should succeed");
        
        // Verify structure is valid
        let integrity = verify_data_integrity(&encrypted);
        assert!(integrity.is_ok() && integrity.unwrap(), "Integrity check should succeed for valid data");
        
        // Test with valid JSON but invalid structure
        let invalid = r#"{"not_cipher":"test","not_iv":"test"}"#;
        let integrity = verify_data_integrity(invalid);
        assert!(integrity.is_err(), "Integrity check should fail for invalid structure");
    }
    
    #[wasm_bindgen_test]
    fn test_large_data_handling() {
        // Create a larger JSON document
        let mut large_data = String::from(r#"{"player_id":"large_test","items":["#);
        for i in 0..100 {
            if i > 0 {
                large_data.push_str(",");
            }
            large_data.push_str(&format!(r#"{{"id":{},"name":"Item {}","value":{}}}"#, i, i, i * 10));
        }
        large_data.push_str("]}");
        
        // Encrypt and decrypt
        let encrypted = encrypt_data(&large_data).expect("Encryption of large data should succeed");
        let decrypted = decrypt_data(&encrypted).expect("Decryption of large data should succeed");
        
        // Verify round trip
        assert_eq!(decrypted, large_data, "Large data should survive round trip");
    }
    
    #[wasm_bindgen_test]
    fn test_special_characters() {
        // Test with special characters and unicode
        let special_data = r#"{"player_id":"unicode_test","name":"✓ Special 😀 Characters! ß","quotes":"\"Quotes\" and 'apostrophes'"}"#;
        
        // Encrypt and decrypt
        let encrypted = encrypt_data(special_data).expect("Encryption with special chars should succeed");
        let decrypted = decrypt_data(&encrypted).expect("Decryption with special chars should succeed");
        
        // Verify round trip
        assert_eq!(decrypted, special_data, "Special characters should survive round trip");
    }
}