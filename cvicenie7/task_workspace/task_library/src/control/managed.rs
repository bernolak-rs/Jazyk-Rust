use crate::task::{ReadTaskFromUser, Task, TaskManager};

pub fn list_tasks(tm: &TaskManager) {
    tm.print_all_tasks();
}

pub fn show_task_by_id(tm: &TaskManager, task_id: usize) {
    // podla id najdem task
    let task = tm.get_task_by_id(task_id);
    // vypisem ho
    match task {
        Some(task) => task.print_task(),
        None => println!("Task so zadanym ID nebol najdeny."),
    }
}

pub fn create_task(reader: &impl ReadTaskFromUser) -> Task {
    Task {
        id: reader.read_id("Zadaj ID tasku: "),
        nazov: reader.read_nazov("Zadaj nazov tasku: "),
        popis: reader.read_popis("Zadaj popis tasku: "),
        priorita: reader.read_priorita("Zadaj prioritu: "),
        planovany_zaciatok: reader.read_planovany_zaciatok("Zadaj planovany zaciatok: "),
        skutocny_zaciatok: reader.read_skutocny_zaciatok("Zadaj skutocny zaciatok: "),
        planovane_trvanie: reader.read_planovane_trvanie("Zadaj planovane trvanie: "),
        skutocne_trvanie: reader.read_skutocne_trvanie("Zadaj skutocne trvanie: "),
    }
}

pub fn add_task(tm: &mut TaskManager, reader: &impl ReadTaskFromUser) {
    tm.add_task(create_task(reader));
}

pub fn remove_task_by_id(tm: &mut TaskManager, task_id: usize) {
    let result = tm.remove_by_id(task_id);
    match result {
        Some(_) => println!("Task s ID {} odstraneny", task_id),
        None => println!("Task s ID {} nebol najdeny", task_id),
    };
}
