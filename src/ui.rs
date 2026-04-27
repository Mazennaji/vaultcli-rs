use colored::*;

pub fn success(message: &str) {
    println!("{}", format!("✓ {}", message).green().bold());
}

pub fn error(message: &str) {
    eprintln!("{}", format!("✕ {}", message).red().bold());
}

pub fn warning(message: &str) {
    println!("{}", format!("! {}", message).yellow().bold());
}

pub fn info(message: &str) {
    println!("{}", format!("→ {}", message).cyan());
}

pub fn title(message: &str) {
    println!();
    println!("{}", message.bold().bright_white());
    println!("{}", "─".repeat(message.len()).bright_black());
}