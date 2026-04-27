use crate::models::{Vault, VaultEntry};

pub fn add_entry(
    vault: &mut Vault,
    title: String,
    username: String,
    password: String,
    website: Option<String>,
    notes: Option<String>,
    category: Option<String>,
) {
    let entry = VaultEntry::new(title, username, password, website, notes, category);
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

pub fn delete_entry(vault: &mut Vault, title: String) -> bool {
    let original_len = vault.entries.len();

    vault
        .entries
        .retain(|entry| entry.title.to_lowercase() != title.to_lowercase());

    vault.entries.len() != original_len
}

pub fn update_password(vault: &mut Vault, title: String, new_password: String) -> bool {
    if let Some(entry) = vault
        .entries
        .iter_mut()
        .find(|entry| entry.title.to_lowercase() == title.to_lowercase())
    {
        entry.password = new_password;
        return true;
    }

    false
}

pub fn summary(vault: &Vault) {
    println!("Vault Summary");
    println!("-------------");
    println!("Total entries: {}", vault.entries.len());

    let with_websites = vault
        .entries
        .iter()
        .filter(|entry| entry.website.is_some())
        .count();

    println!("Entries with websites: {}", with_websites);
}

pub fn list_by_category(vault: &Vault, category: String) {
    let category = category.to_lowercase();

    let results: Vec<&VaultEntry> = vault
        .entries
        .iter()
        .filter(|entry| {
            entry
                .category
                .as_ref()
                .map(|value| value.to_lowercase() == category)
                .unwrap_or(false)
        })
        .collect();

    if results.is_empty() {
        println!("No entries found in this category.");
        return;
    }

    for entry in results {
        println!("{} | {} | {}", entry.id, entry.title, entry.username);
    }
}