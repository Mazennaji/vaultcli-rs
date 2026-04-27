use crate::models::VaultEntry;

pub fn is_weak_password(password: &str) -> bool {
    password.len() < 10
        || password.chars().all(|c| c.is_ascii_lowercase())
        || password.chars().all(|c| c.is_ascii_digit())
        || ["password", "123456", "qwerty", "admin"].contains(&password)
}

pub fn audit_entries(entries: &[VaultEntry]) {
    let weak: Vec<&VaultEntry> = entries
        .iter()
        .filter(|entry| is_weak_password(&entry.password.to_lowercase()))
        .collect();

    if weak.is_empty() {
        println!("No weak passwords found.");
        return;
    }

    println!("Weak passwords found:");

    for entry in weak {
        println!("{} | {}", entry.title, entry.username);
    }
}