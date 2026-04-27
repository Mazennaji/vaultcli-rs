use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

const NONCE_SIZE: usize = 12;

pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], String> {
    let salt = SaltString::from_b64(salt).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    let hash_bytes = hash.hash.ok_or("Failed to derive key")?;

    let mut key = [0u8; 32];
    key.copy_from_slice(&hash_bytes.as_bytes()[..32]);

    Ok(key)
}

pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).as_str().to_string()
}

pub fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| e.to_string())?;

    let mut output = nonce_bytes.to_vec();
    output.extend(encrypted);

    Ok(general_purpose::STANDARD.encode(output))
}

pub fn decrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
    let raw = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| e.to_string())?;

    let nonce = Nonce::from_slice(&raw[..NONCE_SIZE]);
    let ciphertext = &raw[NONCE_SIZE..];

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;

    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Invalid master password or corrupted vault".to_string())?;

    String::from_utf8(decrypted).map_err(|e| e.to_string())
}