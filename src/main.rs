mod audit;
mod crypto;
mod generator;
mod models;
mod storage;
mod vault;

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
    Init,

    Add {
        title: String,
        username: String,
        password: String,

        #[arg(short, long)]
        website: Option<String>,

        #[arg(short, long)]
        notes: Option<String>,
    },

    List,

    Get {
        title: String,
    },

    Search {
        query: String,
    },

    Generate {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
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
}

fn ask_master_password() -> String {
    println!("Enter master password:");
    read_password().expect("Failed to read password")
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("Error: {}", message);
    process::exit(1);
}

fn load_vault_or_exit(master_password: &str) -> models::Vault {
    match storage::load_vault(master_password) {
        Ok(vault) => vault,
        Err(_) => exit_with_error("Invalid master password or corrupted vault."),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let master_password = ask_master_password();

            if let Err(error) = storage::init_vault(&master_password) {
                exit_with_error(&format!("Failed to initialize encrypted vault: {}", error));
            }

            println!("Encrypted vault initialized successfully.");
        }

        Commands::Add {
            title,
            username,
            password,
            website,
            notes,
        } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            vault::add_entry(&mut vault, title, username, password, website, notes);

            if let Err(error) = storage::save_vault(&vault, &master_password) {
                exit_with_error(&format!("Failed to save vault: {}", error));
            }

            println!("Entry added successfully.");
        }

        Commands::List => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::list_entries(&vault);
        }

        Commands::Get { title } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::get_entry(&vault, title);
        }

        Commands::Search { query } => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            vault::search_entries(&vault, query);
        }

        Commands::Generate { length } => {
            if length < 8 {
                exit_with_error("Password length must be at least 8 characters.");
            }

            let password = generator::generate_password(length);
            println!("{}", password);
        }

        Commands::Audit => {
            let master_password = ask_master_password();
            let vault = load_vault_or_exit(&master_password);

            audit::audit_entries(&vault.entries);
        }

        Commands::Delete { title } => {
            let master_password = ask_master_password();
            let mut vault = load_vault_or_exit(&master_password);

            if vault::delete_entry(&mut vault, title) {
                if let Err(error) = storage::save_vault(&vault, &master_password) {
                    exit_with_error(&format!("Failed to save vault: {}", error));
                }

                println!("Entry deleted successfully.");
            } else {
                println!("Entry not found.");
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

                println!("Password updated successfully.");
            } else {
                println!("Entry not found.");
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

            println!("Backup exported successfully.");
        }

        Commands::Import { path } => {
            if let Err(error) = storage::import_backup(&path) {
                exit_with_error(&format!("Failed to import backup: {}", error));
            }

            println!("Backup imported successfully.");
        }
    }
}