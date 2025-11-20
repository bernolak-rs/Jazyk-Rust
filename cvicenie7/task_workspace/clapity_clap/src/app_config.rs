use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct AppConfig {
    pub path: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ListTasks,
    ShowTaskById { task_id: usize },
    AddTask,
    RemoveTaskById { task_id: usize },
    RunInteractive,
}
