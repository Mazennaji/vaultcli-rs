mod models;
mod storage;
mod vault;

use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let vault = models::Vault::default();
            storage::save_vault(&vault).expect("Failed to initialize vault");
            println!("Vault initialized successfully.");
        }

        Commands::Add {
            title,
            username,
            password,
            website,
            notes,
        } => {
            let mut vault = storage::load_vault().expect("Failed to load vault");
            vault::add_entry(&mut vault, title, username, password, website, notes);
            storage::save_vault(&vault).expect("Failed to save vault");
            println!("Entry added successfully.");
        }

        Commands::List => {
            let vault = storage::load_vault().expect("Failed to load vault");
            vault::list_entries(&vault);
        }

        Commands::Get { title } => {
            let vault = storage::load_vault().expect("Failed to load vault");
            vault::get_entry(&vault, title);
        }
    }
}