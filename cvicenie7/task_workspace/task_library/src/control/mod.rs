use std::{fs::File, path::PathBuf};

use crate::task::{ReadTaskFromUser, Task, TaskManager};

pub mod db;
pub mod managed;

pub fn deserialize_json(path: &PathBuf) -> TaskManager {
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

pub fn serialize_json(path: &PathBuf, tm: &TaskManager) {
    let result = serde_json::to_writer(File::create(path).unwrap(), &tm);
    match result {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }
}

pub fn list_tasks(path: &PathBuf) {
    let tm = deserialize_json(path);
    managed::list_tasks(&tm);
}

pub fn show_task_by_id(path: &PathBuf, task_id: usize) {
    let tm = deserialize_json(path);
    managed::show_task_by_id(&tm, task_id);
}

pub fn add_task(path: &PathBuf, reader: &impl ReadTaskFromUser) {
    let mut tm = deserialize_json(path);
    managed::add_task(&mut tm, reader);
    serialize_json(path, &tm);
}

pub fn remove_task_by_id(path: &PathBuf, task_id: usize) {
    let mut tm = deserialize_json(path);
    managed::remove_task_by_id(&mut tm, task_id);
    serialize_json(path, &tm);
}
