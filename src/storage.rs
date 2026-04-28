use crate::crypto;
use crate::models::Vault;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use chrono::Utc;
use crate::vault;

const VAULT_DIR: &str = ".vaultcli";
const VAULT_FILE: &str = "vault.secure";

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedVault {
    salt: String,
    data: String,
}

fn vault_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;

    let path = home.join(VAULT_DIR);

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn vault_path() -> io::Result<PathBuf> {
    Ok(vault_dir()?.join(VAULT_FILE))
}

pub fn vault_exists() -> bool {
    match vault_path() {
        Ok(path) => path.exists(),
        Err(_) => false,
    }
}

pub fn init_vault(master_password: &str) -> io::Result<()> {
    let salt = crypto::generate_salt();

    let key = crypto::derive_key(master_password, &salt)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let vault = Vault::default();
    let json = serde_json::to_string_pretty(&vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let file = EncryptedVault {
        salt,
        data: encrypted,
    };

    let content = serde_json::to_string_pretty(&file)?;
    fs::write(vault_path()?, content)
}

pub fn load_vault(master_password: &str) -> io::Result<Vault> {
    let path = vault_path()?;

    if !path.exists() {
        return Ok(Vault::default());
    }

    let content = fs::read_to_string(path)?;
    let encrypted_vault: EncryptedVault = serde_json::from_str(&content)?;

    let key = crypto::derive_key(master_password, &encrypted_vault.salt)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let decrypted = crypto::decrypt(&encrypted_vault.data, &key)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let vault = serde_json::from_str(&decrypted)?;

    Ok(vault)
}

pub fn save_vault(vault: &Vault, master_password: &str) -> io::Result<()> {
    let path = vault_path()?;
    let content = fs::read_to_string(&path)?;
    let encrypted_vault: EncryptedVault = serde_json::from_str(&content)?;

    let key = crypto::derive_key(master_password, &encrypted_vault.salt)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let json = serde_json::to_string_pretty(vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let updated = EncryptedVault {
        salt: encrypted_vault.salt,
        data: encrypted,
    };

    let output = serde_json::to_string_pretty(&updated)?;
    fs::write(path, output)
}

pub fn export_backup(path: &str) -> io::Result<()> {
    fs::copy(vault_path()?, path)?;
    Ok(())
}

pub fn import_backup(path: &str) -> io::Result<()> {
    fs::copy(path, vault_path()?)?;
    Ok(())
}

pub fn change_master_password(old_password: &str, new_password: &str) -> io::Result<()> {
    let vault = load_vault(old_password)?;

    let salt = crypto::generate_salt();

    let key = crypto::derive_key(new_password, &salt)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let json = serde_json::to_string_pretty(&vault)?;

    let encrypted = crypto::encrypt(&json, &key)
        .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let file = EncryptedVault {
        salt,
        data: encrypted,
    };

    let content = serde_json::to_string_pretty(&file)?;
    fs::write(vault_path()?, content)
}

pub fn auto_backup() -> io::Result<PathBuf> {
    let source = vault_path()?;

    if !source.exists() {
        return Ok(source);
    }

    let backup_dir = vault_dir()?.join("backups");

    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)?;
    }

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("vault_backup_{}.secure", timestamp));

    fs::copy(source, &backup_path)?;

    Ok(backup_path)
}

pub fn export_csv(vault: &Vault, path: &str) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record([
        "id",
        "title",
        "username",
        "password",
        "website",
        "notes",
        "category",
        "created_at",
    ])?;

    for entry in &vault.entries {
        writer.write_record([
            entry.id.to_string(),
            entry.title.clone(),
            entry.username.clone(),
            entry.password.clone(),
            entry.website.clone().unwrap_or_default(),
            entry.notes.clone().unwrap_or_default(),
            entry.category.clone().unwrap_or_default(),
            entry.created_at.to_rfc3339(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn list_backups() -> io::Result<Vec<PathBuf>> {
    let backup_dir = vault_dir()?.join("backups");

    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();

    for entry in fs::read_dir(backup_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("secure") {
            backups.push(path);
        }
    }

    backups.sort();

    Ok(backups)
}

pub fn restore_backup(path: &str) -> io::Result<()> {
    auto_backup()?;
    fs::copy(path, vault_path()?)?;
    Ok(())
}

pub fn import_csv(vault: &mut Vault, path: &str) -> io::Result<usize> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut imported_count = 0;

    for result in reader.records() {
        let record = result?;

        let title = record.get(1).unwrap_or_default().to_string();
        let username = record.get(2).unwrap_or_default().to_string();
        let password = record.get(3).unwrap_or_default().to_string();
        let website = optional_string(record.get(4).unwrap_or_default());
        let notes = optional_string(record.get(5).unwrap_or_default());
        let category = optional_string(record.get(6).unwrap_or_default());

        if title.trim().is_empty() || username.trim().is_empty() || password.trim().is_empty() {
            continue;
        }

        vault::add_entry(vault, title, username, password, website, notes, category);
        imported_count += 1;
    }

    Ok(imported_count)
}

fn optional_string(value: &str) -> Option<String> {
    let value = value.trim();

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}