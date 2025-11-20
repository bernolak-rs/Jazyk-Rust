use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
    usize,
};

use app_config::{AppConfig, Commands};
use clap::Parser;
use console_reader::{ConsoleReader, read_i32};
use task_library::{control, task::ReadTaskFromUser};

use crate::console_reader::read_usize;

mod app_config;
mod console_reader;

fn main() {
    // spustim aplikaciu s prikazom print-tasks - aplikacia nacita tasky z json suboru a vypise ich
    // spustim aplikaciu s prikazom add-task - aplikacia nacita udaje pre novy task od pouzivatela
    // a prida ho do suboru s taskami
    // spustim aplikaciu s prikazom get-task a argumentom id tasku - aplikacia vypise prislusny task
    let mut args = AppConfig::parse();

    // 1. budeme hladat cestu ako argument programu
    args.path = match args.path {
        Some(path) => Some(path),
        None => {
            // 2. ak to nie je medzi argumentami programu, zoberieme to z premennej prostredia
            let path = match std::env::var("TASK_PATH") {
                Ok(p) => p,
                // 3. ak to nie je ani tam, tak pouzijeme nejaku defaultnu cestu (napr. tasky.json)
                Err(_) => String::from("tasky.json"),
            };
            Option::from(PathBuf::from(path))
        }
    };

    // 1. pri moznosti ListTasks, ak pouzivatel zada prepinac --show-header, zobrazi sa okrem zoznamu taskov
    // aj hlavicka - popis jednotlivych stlpcov, ktore sa vypisuju

    // 2. pri pridavani taskov upravit nacitavanie takym sposobom, ze ked pouzivatel zada "-"
    // pri nepovinnych atributoch, tak tieto sa nastavia na hodnotu None

    match args.command {
        Commands::ListTasks => control::list_tasks(&args.path.unwrap()),
        Commands::ShowTaskById { task_id } => {
            control::show_task_by_id(&args.path.unwrap(), task_id)
        }
        Commands::AddTask => control::add_task(&args.path.unwrap(), &ConsoleReader),
        Commands::RemoveTaskById { task_id } => {
            control::remove_task_by_id(&args.path.unwrap(), task_id)
        }
        Commands::RunInteractive => run_in_interactive_mode(&args.path.unwrap(), ConsoleReader),
    };
}

fn run_in_interactive_mode(path: &PathBuf, reader: impl ReadTaskFromUser) {
    let mut tm = control::deserialize_json(path);
    loop {
        print!("Dostupne prikazy: [list, add, remove, show, quit]: ");
        stdout().flush().expect("Error while flushing");
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        let buffer = buffer.trim();
        match buffer {
            "quit" => break,
            "list" => control::managed::list_tasks(&tm),
            "add" => control::managed::add_task(&mut tm, &reader),
            "remove" => control::managed::remove_task_by_id(
                &mut tm,
                read_usize("Zadaj id tasku na odstranenie: "),
            ),
            "show" => {
                control::managed::show_task_by_id(&tm, read_usize("Zadaj id tasku na zobrazenie: "))
            }
            _ => println!("Neplatny prikaz."),
        };
    }
    control::serialize_json(path, &tm);
}
