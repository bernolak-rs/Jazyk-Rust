use std::io::{Write, stdin, stdout};

use chrono::{NaiveDate, TimeDelta};
use task_library::task::ReadTask;

pub struct ConsoleReader;
impl ReadTask for ConsoleReader {
    fn read_id(&self, message: &str) -> i32 {
        read_number(message)
    }

    fn read_name(&self, message: &str) -> String {
        read_string(message)
    }

    fn read_description(&self, message: &str) -> String {
        read_string(message)
    }

    fn read_priority(&self, message: &str) -> i32 {
        read_number(message)
    }

    fn read_planned_from(&self, message: &str) -> chrono::NaiveDate {
        read_date(message)
    }

    fn read_planned_duration(&self, message: &str) -> chrono::TimeDelta {
        read_timedelta(message)
    }

    fn read_real_from(&self, message: &str) -> Option<chrono::NaiveDate> {
        read_date_optional(message)
    }

    fn read_real_duration(&self, message: &str) -> Option<chrono::TimeDelta> {
        read_timedelta_optional(message)
    }
}

pub fn read_string(message: &str) -> String {
    print!("{message}");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Cannot read from console");

    buf.trim().to_string()
}

fn read_number(message: &str) -> i32 {
    let string = read_string(message);
    string
        .trim()
        .parse()
        .expect("Couldn't convert {message} to i32")
}

fn parse_date(string: &str) -> NaiveDate {
    NaiveDate::parse_from_str(string, "%d.%m.%Y").expect("Couldn't convert {string} to date")
}

fn read_date(message: &str) -> NaiveDate {
    let string = read_string(message);
    parse_date(&string)
}

fn read_date_optional(message: &str) -> Option<NaiveDate> {
    let string = read_string(message);
    let string = string.trim();

    if string == "-" || string.is_empty() {
        return None;
    }

    Some(parse_date(string))
}

fn parse_timedelta(string: &str) -> TimeDelta {
    let number: i32 = string
        .trim()
        .parse()
        .expect("Couldn't convert {string} to i32");

    TimeDelta::days(number.into())
}

fn read_timedelta(message: &str) -> TimeDelta {
    let string = read_string(message);
    parse_timedelta(&string)
}

fn read_timedelta_optional(message: &str) -> Option<TimeDelta> {
    let string = read_string(message);
    let string = string.trim();

    if string == "-" || string.is_empty() {
        return None;
    }

    Some(parse_timedelta(string))
}
