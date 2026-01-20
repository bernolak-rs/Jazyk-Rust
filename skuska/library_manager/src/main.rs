use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Book {
    id: usize,
    title: String,
    author: String,
    year: i32,
    is_borrowed: bool,
    borrowed_at: Option<DateTime<Local>>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Borrow(BorrowArgs),
    Return(ReturnArgs),
    List(ListArgs),
}

#[derive(Args)]
struct AddArgs {
    title: String,
    author: String,
    #[arg(short, long)]
    year: i32,
}

#[derive(Args)]
struct BorrowArgs {
    id: usize,
}

#[derive(Args)]
struct ReturnArgs {
    id: usize,
}

#[derive(Args)]
struct ListArgs {
    borrowed: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = "library.json";
}
