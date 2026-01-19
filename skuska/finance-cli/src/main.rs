use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand};
use core::panic;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: usize,
    amount: f64,
    category: String,
    description: Option<String>,
    date: DateTime<Local>,
}

#[derive(Parser)]
#[command(author, version, about = "jednoduchy spravca financii")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    List,
    Stats,
}

#[derive(Args)]
struct AddArgs {
    #[arg(short, long)]
    amount: f64,

    #[arg(short, long)]
    category: String,

    #[arg(short, long)]
    description: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let file_path = "history.json";

    match cli.command {
        Commands::Add(args) => {
            let mut transactions = load_transactions(file_path);

            let new_transaction = Transaction {
                id: transactions.len() + 1,
                amount: args.amount,
                category: args.category,
                description: args.description,
                date: Local::now(),
            };

            transactions.push(new_transaction);
            save_transactions(file_path, &transactions);
            println!("Transakcia uspesne pridana.")
        }
        Commands::List => {
            let mut transactions = load_transactions(file_path);

            for t in transactions {
                println!(
                    "[{}] {} $ - {} ({:?})",
                    t.date.format("%d.%m.%Y %H:%M"),
                    t.amount,
                    t.category,
                    t.description.unwrap_or_default()
                );
            }
        }
        Commands::Stats => {
            println!("Logika statistiky...")
        }
    }
}

fn load_transactions(path: &str) -> Vec<Transaction> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or(0);
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_transactions(path: &str, data: &Vec<Transaction>) {
    let json = serde_json::to_string_pretty(data).expect("Chyba pri serializacii");
    let mut file = File::create(path).expect("Nepodarilo sa vytvorit subor");
    file.write_all(json.as_bytes()).expect("Chyba pri zapise");
}
