mod audit;
mod crypto;
mod generator;
mod models;
mod storage;
mod vault;

use clap::{Parser, Subcommand};
use rpassword::read_password;

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
}

fn ask_master_password() -> String {
    println!("Enter master password:");
    read_password().expect("Failed to read password")
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let master_password = ask_master_password();

            storage::init_vault(&master_password).expect("Failed to initialize encrypted vault");

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

            let mut vault = storage::load_vault(&master_password).expect("Failed to load vault");

            vault::add_entry(&mut vault, title, username, password, website, notes);

            storage::save_vault(&vault, &master_password).expect("Failed to save vault");

            println!("Entry added successfully.");
        }

        Commands::List => {
            let master_password = ask_master_password();

            let vault = storage::load_vault(&master_password).expect("Failed to load vault");

            vault::list_entries(&vault);
        }

        Commands::Get { title } => {
            let master_password = ask_master_password();

            let vault = storage::load_vault(&master_password).expect("Failed to load vault");

            vault::get_entry(&vault, title);
        }

        Commands::Search { query } => {
            let master_password = ask_master_password();

            let vault = storage::load_vault(&master_password).expect("Failed to load vault");

            vault::search_entries(&vault, query);
        }

        Commands::Generate { length } => {
            let password = generator::generate_password(length);
            println!("{}", password);
        }

        Commands::Audit => {
            let master_password = ask_master_password();

            let vault = storage::load_vault(&master_password).expect("Failed to load vault");

            audit::audit_entries(&vault.entries);
        }
    }
}