use crate::crypto;
use crate::models::Vault;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

const VAULT_FILE: &str = "vault.secure";

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedVault {
    salt: String,
    data: String,
}

pub fn init_vault(master_password: &str) -> io::Result<()> {
    let salt = crypto::generate_salt();
    let key = crypto::derive_key(master_password, &salt)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let vault = Vault::default();
    let json = serde_json::to_string_pretty(&vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let file = EncryptedVault {
        salt,
        data: encrypted,
    };

    let content = serde_json::to_string_pretty(&file)?;
    fs::write(VAULT_FILE, content)
}

pub fn load_vault(master_password: &str) -> io::Result<Vault> {
    if !Path::new(VAULT_FILE).exists() {
        return Ok(Vault::default());
    }

    let content = fs::read_to_string(VAULT_FILE)?;
    let encrypted_vault: EncryptedVault = serde_json::from_str(&content)?;

    let key = crypto::derive_key(master_password, &encrypted_vault.salt)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let decrypted = crypto::decrypt(&encrypted_vault.data, &key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let vault = serde_json::from_str(&decrypted)?;

    Ok(vault)
}

pub fn save_vault(vault: &Vault, master_password: &str) -> io::Result<()> {
    let content = fs::read_to_string(VAULT_FILE)?;
    let encrypted_vault: EncryptedVault = serde_json::from_str(&content)?;

    let key = crypto::derive_key(master_password, &encrypted_vault.salt)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let json = serde_json::to_string_pretty(vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let updated = EncryptedVault {
        salt: encrypted_vault.salt,
        data: encrypted,
    };

    let output = serde_json::to_string_pretty(&updated)?;
    fs::write(VAULT_FILE, output)
}

pub fn export_backup(path: &str) -> io::Result<()> {
    fs::copy(VAULT_FILE, path)?;
    Ok(())
}

pub fn import_backup(path: &str) -> io::Result<()> {
    fs::copy(path, VAULT_FILE)?;
    Ok(())
}

pub fn change_master_password(old_password: &str, new_password: &str) -> io::Result<()> {
    let vault = load_vault(old_password)?;

    let salt = crypto::generate_salt();
    let key = crypto::derive_key(new_password, &salt)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let json = serde_json::to_string_pretty(&vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let file = EncryptedVault {
        salt,
        data: encrypted,
    };

    let content = serde_json::to_string_pretty(&file)?;
    fs::write(VAULT_FILE, content)
}