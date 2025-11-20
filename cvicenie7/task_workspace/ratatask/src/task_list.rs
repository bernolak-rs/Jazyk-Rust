use std::path::PathBuf;

use ratatui::widgets::ListState;
use task_library::{
    control::deserialize_json,
    task::{Task, TaskManager},
};

#[derive(Debug)]
pub struct TaskList {
    pub task_manager: TaskManager,
    pub state: ListState,
    pub rendered_task: Option<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            task_manager: TaskManager::new(),
            state: ListState::default(),
            rendered_task: None,
        }
    }

    pub fn open(mut self, path: &PathBuf) -> Self {
        self.task_manager = deserialize_json(path);
        self
    }

    pub fn update_selected_task(&mut self) {
        self.rendered_task = self
            .state
            .selected()
            .and_then(|i| self.task_manager.get_tasks().get(i).cloned());
    }
}

impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
