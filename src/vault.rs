use crate::models::{Vault, VaultEntry};

pub fn add_entry(
    vault: &mut Vault,
    title: String,
    username: String,
    password: String,
    website: Option<String>,
    notes: Option<String>,
) {
    let entry = VaultEntry::new(title, username, password, website, notes);
    vault.entries.push(entry);
}

pub fn list_entries(vault: &Vault) {
    if vault.entries.is_empty() {
        println!("No entries found.");
        return;
    }

    for entry in &vault.entries {
        println!("{} | {} | {}", entry.id, entry.title, entry.username);
    }
}

pub fn get_entry(vault: &Vault, title: String) {
    let found = vault
        .entries
        .iter()
        .find(|entry| entry.title.to_lowercase() == title.to_lowercase());

    match found {
        Some(entry) => {
            println!("Title: {}", entry.title);
            println!("Username: {}", entry.username);
            println!("Password: {}", entry.password);

            if let Some(website) = &entry.website {
                println!("Website: {}", website);
            }

            if let Some(notes) = &entry.notes {
                println!("Notes: {}", notes);
            }
        }
        None => println!("Entry not found."),
    }
}

pub fn search_entries(vault: &Vault, query: String) {
    let query = query.to_lowercase();

    let results: Vec<&VaultEntry> = vault
        .entries
        .iter()
        .filter(|entry| {
            entry.title.to_lowercase().contains(&query)
                || entry.username.to_lowercase().contains(&query)
                || entry
                    .website
                    .as_ref()
                    .map(|website| website.to_lowercase().contains(&query))
                    .unwrap_or(false)
        })
        .collect();

    if results.is_empty() {
        println!("No matching entries found.");
        return;
    }

    for entry in results {
        println!("{} | {} | {}", entry.id, entry.title, entry.username);
    }
}