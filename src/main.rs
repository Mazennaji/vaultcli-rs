mod audit;
mod crypto;
mod generator;
mod models;
mod storage;
mod ui;
mod vault;
mod clipboard;
mod tui;

use clap::{Parser, Subcommand};
use rpassword::read_password;
use std::process;

#[derive(Parser)]
#[command(name = "vaultcli")]
#[command(about = "Encrypted password vault CLI built in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },

    Add {
        title: String,
        username: String,
        password: String,

        #[arg(short, long)]
        website: Option<String>,

        #[arg(short, long)]
        notes: Option<String>,

        #[arg(short, long)]
        category: Option<String>,
    },

    List,

    Get {
        title: String,

        #[arg(short, long, default_value_t = false)]
        reveal: bool,
    },

    Search {
        query: String,
    },

    Generate {
        #[arg(short, long, default_value_t = 16)]
        length: usize,

        #[arg(long, default_value_t = true)]
        numbers: bool,

        #[arg(long, default_value_t = false)]
        symbols: bool,
    },

    Audit,

    Delete {
        title: String,
    },

    UpdatePassword {
        title: String,
        new_password: String,
    },

    Summary,

    Export {
        path: String,
    },

    Import {
        path: String,
    },

    ChangeMaster,

    Category {
        category: String,
    },

    Copy {
        title: String,
    },

    Tui,

    Config,

    ExportCsv {
        path: String,
    },

    Backups,

    Restore {
        path: String,
    },

    ImportCsv {
        path: String,
    },

    Strength {
        password: String,
    },
}

fn ask_master_password() -> String {
    println!("Enter master password:");
    read_password().expect("Failed to read password")
}

fn ask_confirmed_master_password() -> String {
    println!("Enter new master password:");
    let password = read_password().expect("Failed to read password");

    println!("Confirm new master password:");
    let confirmation = read_password().expect("Failed to read password");

    if password != confirmation {
        exit_with_error("Master passwords do not match.");
    }

    if password.len() < 8 {
        exit_with_error("Master password must be at least 8 characters.");
    }

    password
}

fn exit_with_error(message: &str) -> ! {
    ui::error(message);
    process::exit(1);
}

fn load_vault_or_exit(master_password: &str) -> models::Vault {
    match storage::load_vault(master_password) {
        Ok(vault) => vault,
        Err(_) => exit_with_error("Invalid master password or corrupted vault."),
    }
}

fn confirm_action(message: &str) -> bool {
    println!("{} Type YES to confirm:", message);

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim() == "YES"
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            if storage::vault_exists() && !force {
                exit_with_error("Vault already exists. Use --force to overwrite it.");
            }

            if storage::vault_exists()
                && force
                && !confirm_action("This will overwrite your existing vault.")
            {
                ui::warning("Init cancelled.");
                return;
            }

            let master_password = ask_confirmed_master_password();

            if let Err(error) = storage::init_vault(&master_password) {
                exit_with_error(&format!("Failed to initialize encrypted vault: {}", error));
            }

            ui::success("Encrypted vault initialized successfully.");
        }

        Commands::Add {
            title,
            username,
            password,
            website,
            notes,
            category,
        } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            vault::add_entry(
                &mut vault, title, username, password, website, notes, category,
            );

            if let Err(error) = storage::save_vault(&vault, &master_password) {
                exit_with_error(&format!("Failed to save vault: {}", error));
            }

            ui::success("Entry added successfully.");
        }

        Commands::List => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::list_entries(&vault);
        }

        Commands::Get { title, reveal } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::get_entry(&vault, title, reveal);
        }

        Commands::Search { query } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::search_entries(&vault, query);
        }

        Commands::Generate {
            length,
            numbers,
            symbols,
        } => {
            if length < 8 {
                exit_with_error("Password length must be at least 8 characters.");
            }

            let password = generator::generate_password(length, numbers, symbols);
            let strength = audit::password_strength(&password);

            ui::title("Generated Password");
            println!("Password: {}", password);
            println!("Strength: {}", strength);
        }

        Commands::Audit => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            audit::audit_entries(&vault.entries);
        }

        Commands::Delete { title } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            if !confirm_action("This will delete the entry permanently.") {
                ui::warning("Delete cancelled.");
                return;
            }

            if vault::delete_entry(&mut vault, title) {
                if let Err(error) = storage::auto_backup() {
                    exit_with_error(&format!("Failed to create backup: {}", error));
                }

                ui::success("Entry deleted successfully.");
            } else {
                ui::warning("Entry not found.");
            }
        }

        Commands::UpdatePassword {
            title,
            new_password,
        } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            if vault::update_password(&mut vault, title, new_password) {
                if let Err(error) = storage::save_vault(&vault, &master_password) {
                    exit_with_error(&format!("Failed to save vault: {}", error));
                }

                ui::success("Password updated successfully.");
            } else {
                ui::warning("Entry not found.");
            }
        }

        Commands::Summary => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::summary(&vault);
        }

        Commands::Export { path } => {
            if let Err(error) = storage::export_backup(&path) {
                exit_with_error(&format!("Failed to export backup: {}", error));
            }

            ui::success("Backup exported successfully.");
        }

        Commands::Import { path } => {
            if let Err(error) = storage::import_backup(&path) {
                exit_with_error(&format!("Failed to import backup: {}", error));
            }

            ui::success("Backup imported successfully.");
        }

        Commands::ChangeMaster => {
            let old_password = ask_master_password();
            let new_password = ask_confirmed_master_password();

            if storage::change_master_password(&old_password, &new_password).is_err() {
                exit_with_error("Invalid old master password or corrupted vault.");
            }

            ui::success("Master password changed successfully.");
        }

        Commands::Category { category } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::list_by_category(&vault, category);
        }

        Commands::Copy { title } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            match vault::find_entry_by_title(&vault, &title) {
                Some(entry) => {
                    if let Err(error) = clipboard::copy_to_clipboard(&entry.password) {
                        exit_with_error(&format!("Failed to copy password: {}", error));
                    }

                    ui::success("Password copied to clipboard.");
                }
                None => ui::warning("Entry not found."),
            }
        }

        Commands::Tui => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            if let Err(error) = tui::run_tui(&mut vault, &master_password) {
                exit_with_error(&format!("Failed to launch TUI: {}", error));
            }
        }

        Commands::Config => {
            match storage::vault_path() {
                Ok(path) => {
                    ui::title("VaultCLI Config");
                    println!("Vault path: {}", path.display());
                }
                Err(error) => {
                    exit_with_error(&format!("Failed to read config: {}", error));
                }
            }
        }

        Commands::ExportCsv { path } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            if let Err(error) = storage::export_csv(&vault, &path) {
                exit_with_error(&format!("Failed to export CSV: {}", error));
            }

            ui::success("CSV exported successfully.");
        }

        Commands::Backups => {
            match storage::list_backups() {
                Ok(backups) => {
                    if backups.is_empty() {
                        ui::warning("No backups found.");
                        return;
                    }

                    ui::title("Vault Backups");

                    for backup in backups {
                        println!("{}", backup.display());
                    }
                }
                Err(error) => {
                    exit_with_error(&format!("Failed to list backups: {}", error));
                }
            }
        }

        Commands::Restore { path } => {
            if !confirm_action("This will replace your current vault with the selected backup.") {
                ui::warning("Restore cancelled.");
                return;
            }

            if let Err(error) = storage::restore_backup(&path) {
                exit_with_error(&format!("Failed to restore backup: {}", error));
            }

            ui::success("Backup restored successfully.");
        }

        Commands::ImportCsv { path } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            let imported_count = match storage::import_csv(&mut vault, &path) {
                Ok(count) => count,
                Err(error) => exit_with_error(&format!("Failed to import CSV: {}", error)),
            };

            if let Err(error) = storage::save_vault(&vault, &master_password) {
                exit_with_error(&format!("Failed to save vault: {}", error));
            }

            ui::success(&format!("Imported {} entries from CSV.", imported_count));
        }

        Commands::Strength { password } => {
            let strength = audit::password_strength(&password);

            ui::title("Password Strength");
            println!("Result: {}", strength);
        }
    }
}
