//! Ovládanie aplikácie (main.rs)
//
// Implementujte pomocou knižnice clap s týmito príkazmi:
//
//     daj-knihu-isbn: (2 argumenty: cesta k súboru, isbn) – Vyhľadá a vypíše knihu.
//
//     daj-knihy-autora: (2 argumenty: cesta k súboru, autor) – Vyhľadá a vypíše knihy autora.
//
//     odstran-knihu: (2 argumenty: cesta k súboru, isbn) – Odstráni knihu a uloží aktualizovanú knižnicu.
//
//     vypis-vydavatelstva: (1 argument: cesta k súboru) – Vypíše štatistiky vydavateľstiev.
mod lib;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::lib::Kniznica;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DajKnihuIsbn(DajKnihuIsbnArgs),
    DajKnihuAutora(DajKnihuAutoraArgs),
    OdstranKnihu(OdstranKnihuArgs),
    VypisVydavatelstva(VypisVydavatelstvaArgs),
}

#[derive(Args)]
struct DajKnihuIsbnArgs {
    #[arg(long)]
    cesta_k_suboru: String,

    #[arg(long)]
    isbn: String,
}

#[derive(Args)]
struct DajKnihuAutoraArgs {
    #[arg(long)]
    cesta_k_suboru: String,

    #[arg(long)]
    autor: String,
}

#[derive(Args)]
struct OdstranKnihuArgs {
    #[arg(long)]
    cesta_k_suboru: String,

    #[arg(long)]
    isbn: String,
}

#[derive(Args)]
struct VypisVydavatelstvaArgs {
    #[arg(long)]
    cesta_k_suboru: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::DajKnihuIsbn(daj_knihu_isbn_args) => {
            let mut cesta = PathBuf::new();
            cesta.push(&daj_knihu_isbn_args.cesta_k_suboru);

            let kniznica =
                Kniznica::nacitaj_zo_suboru(&cesta).expect("Nepodarilo sa nacitat kniznicu");
            let kniha = kniznica
                .daj_knihu_podla_isbn(&daj_knihu_isbn_args.isbn)
                .unwrap();
            println!("{}", kniha);
        }
        Commands::DajKnihuAutora(daj_knihu_autora_args) => {
            let mut cesta = PathBuf::new();
            cesta.push(&daj_knihu_autora_args.cesta_k_suboru);

            let kniznica =
                Kniznica::nacitaj_zo_suboru(&cesta).expect("Nepodarilo sa nacitat kniznicu");
            let knihy = kniznica.daj_knihy_autora(&daj_knihu_autora_args.autor);
            for k in knihy {
                println!("{}", k);
            }
        }
        Commands::OdstranKnihu(odstran_knihu_args) => {
            let mut cesta = PathBuf::new();
            cesta.push(&odstran_knihu_args.cesta_k_suboru);

            let mut kniznica =
                Kniznica::nacitaj_zo_suboru(&cesta).expect("Nepodarilo sa nacitat kniznicu");
            let kniha = kniznica.odstran_knihu(&odstran_knihu_args.isbn).unwrap();
            println!("{}", kniha);
            if kniznica.uloz_do_suboru(&cesta) {
                "Kniha bola odstanena"
            } else {
                "Knihu sa nepodarilo odstranit"
            };
        }
        Commands::VypisVydavatelstva(vypis_vydavatelstva_args) => {
            let mut cesta = PathBuf::new();
            cesta.push(&vypis_vydavatelstva_args.cesta_k_suboru);

            let kniznica =
                Kniznica::nacitaj_zo_suboru(&cesta).expect("Nepodarilo sa nacitat kniznicu");
            kniznica.vypis_vydavatelstva_a_pocet_knih();
        }
    }
}
