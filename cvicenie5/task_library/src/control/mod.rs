use std::{fs::File, path::PathBuf};

use crate::task::{create_task_from_console, TaskManager};

fn deserialize_json(path: &PathBuf) -> TaskManager {
    let tm: TaskManager;
    match std::fs::exists(path) {
        Ok(true) => {
            tm = serde_json::from_reader(File::open(path).unwrap()).unwrap();
        }
        Ok(false) => {
            println!(
                "Subor s taskami na zadanej ceste neexistuje. Zadana cesta bola {}",
                path.to_str().unwrap()
            );
            println!("Vytvaram novy task manager.");
            tm = TaskManager::new();
        }
        Err(e) => panic!("{e}"),
    }
    tm
}

fn serialize_json(path: &PathBuf, tm: &TaskManager) {
    let result = serde_json::to_writer(File::create(path).unwrap(), &tm);
    match result {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}

pub fn list_tasks(path: &PathBuf, show_header: bool) {
    if show_header {
        println!("ID | Name | Priority | Planned from | ...");
        println!("------------------------------------------");
    }
    let tm = deserialize_json(path);
    tm.print_tasks();
}

pub fn show_task_by_id(path: &PathBuf, task_id: i32) {
    let tm = deserialize_json(path);
    // podla id najdem task
    let task = tm.get_task_by_id(task_id);
    // vypisem ho
    match task {
        Some(task) => task.print_task(),
        None => println!("Task so zadanym ID nebol najdeny."),
    }
}

pub fn add_task(path: &PathBuf) {
    let mut tm = deserialize_json(path);

    let task = create_task_from_console();
    tm.add_task(task);

    serialize_json(path, &tm);
}

pub fn remove_task_by_id(path: &PathBuf, task_id: i32) {
    let mut tm = deserialize_json(path);

    tm.remove_task(task_id);
    serialize_json(path, &tm);
}

fn print_options() {
    println!(
        "Select option <1-4>:\n\t
        1. Print tasks\n\t
        2. Show task by id\n\t
        3. Add task\n\t
        4. Remove task\n\t
        q for quit\n"
    );
}

pub fn interactive_mode() {
    let input = String::new();

    while input != "q" {}
}
