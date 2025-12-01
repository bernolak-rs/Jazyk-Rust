use crate::schema::tasks::dsl::{id, tasks};
use crate::task::{ReadTaskFromUser, Task};
use crate::{db::estabilish_connection, models::TaskDb, task::TaskManager};
use diesel::associations::HasTable;
use diesel::{connection, prelude::*};

use super::managed;

pub fn create_from_db() -> TaskManager {
    let connection = &mut estabilish_connection();
    let result: Vec<TaskDb> = tasks
        .limit(10)
        .load(connection)
        .expect("Error loading tasks from db");
    let converted = result.iter().map(|t| t.into());
    let mut tm = TaskManager::new();
    for t in converted {
        tm.add_task(t);
    }
    tm
}

pub fn add_task(reader: &impl ReadTaskFromUser) {
    let connection = &mut estabilish_connection();
    let task = managed::create_task(reader);
    let task_db: TaskDb = task.into();
    diesel::insert_into(tasks::table())
        .values(task_db)
        .execute(connection)
        .expect("Error adding task to db!");
}

pub fn remove_task_by_id(task_id: usize) -> bool {
    use diesel::query_dsl::methods::FilterDsl;
    let connection = &mut estabilish_connection();
    let deleted = diesel::delete(FilterDsl::filter(tasks, id.eq(task_id as i32)))
        .execute(connection)
        .expect("Error while deleting task from DB");
    deleted > 0
}

pub fn show_task_by_id(task_id: usize) -> Task {
    use crate::schema::tasks::dsl::{id, tasks};
    use diesel::prelude::*;

    let connection = &mut estabilish_connection();

    let task = tasks
        .filter(id.eq(task_id as i32))
        .first::<TaskDb>(connection)
        .optional()
        .expect("Failed to load task")
        .expect("Failed to load task");

    (&task).into()
}
