use crate::models::Vault;
use std::fs;
use std::io;
use std::path::Path;

const VAULT_FILE: &str = "vault.json";

pub fn load_vault() -> io::Result<Vault> {
    if !Path::new(VAULT_FILE).exists() {
        return Ok(Vault::default());
    }

    let content = fs::read_to_string(VAULT_FILE)?;

    if content.trim().is_empty() {
        return Ok(Vault::default());
    }

    let vault = serde_json::from_str(&content).unwrap_or_default();

    Ok(vault)
}

pub fn save_vault(vault: &Vault) -> io::Result<()> {
    let json = serde_json::to_string_pretty(vault)?;
    fs::write(VAULT_FILE, json)
}