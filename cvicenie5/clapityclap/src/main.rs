use std::
    path::PathBuf
;

use clap::Parser;
use console_reader::ConsoleReader;
use task_library::control;
use app_config::{AppConfig, Commands};

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
                Err(_) => String::from("tasks.json"),
            };
            Option::from(PathBuf::from(path))
        }
    };

    // 1. print moznosti ListTasks, ak pouzivatel zada prepinac --show-header, zobrazi sa okrem
    //    zoznamu taskov aj hlavicka - popis jednotlivych stlpcov, ktore sa vypisuju
    // 2. pri pridavani taskov upravit nacitavanie takym sposobom, ze ked pouzivatel zada "-" pri
    //    nepovinnych atributoch, tak tieto sa nastavia na hodnotu None
    // 3. Pridat Interactive arg ktory do bezi v loope

    match args.command {
        Commands::ListTasks { show_header } => {
            control::list_tasks(&args.path.unwrap(), show_header)
        }
        Commands::ShowTaskById { task_id } => {
            control::show_task_by_id(&args.path.unwrap(), task_id)
        }
        Commands::AddTask => control::add_task(&args.path.unwrap()),
        Commands::RemoveTaskById { task_id } => {
            control::remove_task_by_id(&args.path.unwrap(), task_id)
        }
        Commands::Interactive => control::interactive(),


    // let mut tm = TaskManager::new();
    // tm.read_from_txt_file(Path::new("./tasky.txt"));
    // tm.print_all_tasks();

    // let task = tm.get_task_by_id(5);
    // println!("{task:?}");

    // let serialized_json = serde_json::to_string_pretty(&tm).unwrap();
    // println!("{serialized_json}");
    // let new_task_manager: TaskManager = serde_json::from_str(&serialized_json).unwrap();
    // println!("{new_task_manager:?}");
}
