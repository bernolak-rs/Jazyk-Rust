use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{BufReader, BufWriter},
    path,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Kniha {
    pub autori: Vec<String>,
    pub nazov: String,
    pub vydavatelstvo: String,
    pub zaner: String,
    pub pocet_stran: u32,
    pub isbn: String,
    pub rok_vydania: i32,
    pub je_pozicana: bool,
    pub stav: Stav,
}

impl fmt::Display for Kniha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(
            f,
            "({}, {}, {}, {}, {}, {}, {}, {}, {})",
            self.autori.join(", "),
            self.nazov,
            self.vydavatelstvo,
            self.zaner,
            self.pocet_stran,
            self.isbn,
            self.rok_vydania,
            self.je_pozicana,
            self.stav
        )
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
pub enum Stav {
    #[default]
    Nova,
    Pouzivana,
    Poskodena,
    Vyradena,
}

impl fmt::Display for Stav {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        let s = match self {
            Stav::Nova => "Nova",
            Stav::Pouzivana => "Pouzivana",
            Stav::Poskodena => "Poskodena",
            Stav::Vyradena => "Vyradena",
        };
        write!(f, "({})", s)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Kniznica {
    pub knihy: Vec<Kniha>,
}

impl Kniznica {
    pub fn nacitaj_zo_suboru(cesta: &path::PathBuf) -> Option<Kniznica> {
        let subor = match File::open(cesta) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let reader = BufReader::new(subor);

        match serde_json::from_reader(reader) {
            Ok(kniznica) => Some(kniznica),
            Err(_) => None,
        }
    }

    pub fn uloz_do_suboru(&self, cesta: &path::PathBuf) -> bool {
        let subor = match File::create(cesta) {
            Ok(file) => file,
            Err(_) => return false,
        };

        let writer = BufWriter::new(subor);

        match serde_json::to_writer_pretty(writer, &self) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn pridaj_knihu(&mut self, kniha: Kniha) -> Result<(), ()> {
        if self.knihy.iter().any(|k| k.isbn == kniha.isbn) {
            return Err(());
        }
        self.knihy.push(kniha);
        Ok(())
    }

    pub fn odstran_knihu(&mut self, isbn: &str) -> Result<Kniha, ()> {
        let index = self.knihy.iter().position(|k| k.isbn == isbn);

        match index {
            Some(i) => Ok(self.knihy.remove(i)),
            None => Err(()),
        }
    }

    pub fn daj_knihu_podla_isbn(&self, isbn: &str) -> Option<&Kniha> {
        self.knihy.iter().find(|k| k.isbn == isbn)
    }

    pub fn daj_knihy_autora(&self, autor: &str) -> Vec<&Kniha> {
        self.knihy
            .iter()
            .filter(|k| k.autori.iter().any(|a| a == autor))
            .collect()
    }

    pub fn vypis_vydavatelstva_a_pocet_knih(&self) {
        use std::collections::HashMap;
        let mut statistika = HashMap::new();

        for kniha in &self.knihy {
            *statistika.entry(&kniha.vydavatelstvo).or_insert(0) += 1;
        }

        for (vydavatelstvo, pocet) in statistika {
            println!("{}: {}", vydavatelstvo, pocet);
        }
    }

    pub fn vypis_knihy_daneho_zanru(&self, zaner: &str) {
        for kniha in self.knihy.iter().filter(|k| k.zaner == zaner) {
            println!("{}", kniha);
        }
    }
}

// Ak chceš...,Metóda,Vráti
// Vykonať operáciu(napísanú v predikáte) na každom elemente, map(), Iterator (treba collect)
// "Zistiť, či existuje (Áno/Nie)", any(), bool
// Získať dáta na čítanie,find(), Option<&T>
// Zistiť index (na mazanie), position(), Option<usize>
// Získať viacero výsledkov, filter(), Iterator (treba collect)
