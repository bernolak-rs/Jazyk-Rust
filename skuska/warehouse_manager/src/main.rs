use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand, command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Product {
    id: usize,
    name: String,
    quantity: u32,
    price: f32,
    last_restock: DateTime<Local>,
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
    Restock(RestockArgs),
    Inventory,
    Export,
}

#[derive(Args)]
struct AddArgs {
    name: String,
    #[arg(short, long)]
    price: f32,
}

#[derive(Args)]
struct RestockArgs {
    id: usize,
    quantity: u32,
}

fn main() {
    let cli = Cli::parse();
    let file_path = "warehouse.json";

    let mut inventory = load_inventory(file_path);

    match cli.command {
        Commands::Add(add_args) => add_product(&add_args.name, add_args.price, file_path),
        Commands::Restock(restock_args) => {
            restock_product(restock_args.id, restock_args.quantity, file_path)
        }
        Commands::Inventory => print_inventory(&inventory),
        Commands::Export => save_to_json(file_path, &inventory),
    }
}

fn load_inventory(path: &str) -> Vec<Product> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or(0);
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_to_json(path: &str, inventory: &Vec<Product>) {
    let json = serde_json::to_string_pretty(inventory).expect("Chyba pri serializacii");
    let mut file = File::create(path).expect("Nepodarilo sa vytvorit subor");
    file.write_all(json.as_bytes()).expect("Chyba pri zapise");
}

fn print_inventory(inventory: &Vec<Product>) {
    for i in inventory {
        println!("{} - {}$: [{}]", i.id, i.price, i.quantity);
    }
}

fn restock_product(id: usize, quantity: u32, path: &str) {
    let mut invetory = load_inventory(path);
    if let Some(product) = invetory.iter_mut().find(|p| p.id == id) {
        product.quantity += quantity;
        product.last_restock = Local::now();
        println!("Product was restocted: {}.", product.name);
    } else {
        println!("Product was not found.");
    }
    let mut found = false;

    // Ugly version
    // for product in &mut invetory {
    //     if product.id == id {
    //         product.quantity += quantity;
    //         product.last_restock = Local::now();
    //         found = true;
    //         break;
    //     }
    // }
    //
    // if !found {
    //     println!("Produkt nenájdený.");
    // }
}

fn add_product(name: &str, price: f32, path: &str) {
    let mut inventory = load_inventory(path);
    let new_product = Product {
        id: inventory.len() + 1,
        name: name.into(),
        quantity: 0,
        price,
        last_restock: Local::now(),
    };
    inventory.push(new_product);
    save_to_json(path, &inventory);
}
