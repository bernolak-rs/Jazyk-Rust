use std::io::{Write, stdin, stdout};

use chrono::{NaiveDate, TimeDelta};
use task_library::task::ReadTaskFromUser;

pub struct ConsoleReader;

pub fn read_string(message: &str) -> String {
    print!("{message}");
    stdout().flush().expect("Error while flushing");
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer
}
// nacitanie cisla
pub fn read_i32(message: &str) -> i32 {
    let mut buffer = read_string(message);
    let cislo = buffer.trim().parse().expect("Cannot be parsed to i32");
    cislo
}
// nacitanie cisla
pub fn read_usize(message: &str) -> usize {
    let mut buffer = read_string(message);
    let cislo = buffer.trim().parse().expect("Cannot be parsed to usize");
    cislo
}
// nacitanie datumu
pub fn read_date(message: &str) -> NaiveDate {
    let mut buffer = read_string(message);
    let datum =
        NaiveDate::parse_from_str(buffer.trim(), "%d.%m.%Y").expect("Cannot parse from string");
    datum
}

impl ReadTaskFromUser for ConsoleReader {
    fn read_id(&self, message: &str) -> usize {
        read_usize(message)
    }

    fn read_nazov(&self, message: &str) -> String {
        read_string(message)
    }

    fn read_popis(&self, message: &str) -> String {
        read_string(message)
    }

    fn read_priorita(&self, message: &str) -> i32 {
        read_i32(message)
    }

    fn read_planovany_zaciatok(&self, message: &str) -> chrono::NaiveDate {
        read_date(message)
    }

    fn read_planovane_trvanie(&self, message: &str) -> chrono::TimeDelta {
        TimeDelta::days(read_i32(message).into())
    }

    fn read_skutocny_zaciatok(&self, message: &str) -> Option<chrono::NaiveDate> {
        Some(read_date(message))
    }

    fn read_skutocne_trvanie(&self, message: &str) -> Option<chrono::TimeDelta> {
        Some(TimeDelta::days(read_i32(message).into()))
    }
}
