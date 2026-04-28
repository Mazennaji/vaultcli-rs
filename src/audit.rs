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

pub fn password_strength(password: &str) -> String {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }

    if password.len() >= 12 {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_lowercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_uppercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }

    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=2 => "Weak".to_string(),
        3..=4 => "Medium".to_string(),
        _ => "Strong".to_string(),
    }
}