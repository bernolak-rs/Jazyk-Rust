use std::{fmt::Display, path::PathBuf};

use crate::{
    event::{AppEvent, Event, EventHandler, TaskListEvent},
    task_list::TaskList,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout},
};
use task_library::task::TaskManager;

#[derive(Debug)]
pub enum FocusedWidget {
    TopBar,
    TaskList,
    TaskDescription,
    Gant,
    Help,
}

impl Display for FocusedWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FocusedWidget::TopBar => write!(f, "Top bar"),
            FocusedWidget::TaskList => write!(f, "Task list"),
            FocusedWidget::TaskDescription => write!(f, "Task description"),
            FocusedWidget::Gant => write!(f, "Gant"),
            FocusedWidget::Help => write!(f, "Help"),
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub focused_widget: FocusedWidget,
    pub task_path: PathBuf,
    pub task_list: TaskList,
}

impl Default for App {
    fn default() -> Self {
        let task_path = PathBuf::from("tasky_argument.json");
        Self {
            running: true,
            events: EventHandler::new(),
            focused_widget: FocusedWidget::TopBar,
            task_list: TaskList::new().open(&task_path),
            task_path: task_path,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::FocusNext => self.focus_next(),
                    AppEvent::FocusPrevious => self.focus_previous(),
                    AppEvent::TaskList(task_list_event) => {
                        self.handle_task_list_event(task_list_event)
                    }
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Tab => self.events.send(AppEvent::FocusNext),
            KeyCode::BackTab => self.events.send(AppEvent::FocusPrevious),
            // Other handlers you could add here.
            _ => {}
        }
        if let FocusedWidget::TaskList = self.focused_widget {
            match key_event.code {
                KeyCode::Esc => self
                    .events
                    .send(AppEvent::TaskList(TaskListEvent::Deselect)),
                KeyCode::Enter => self
                    .events
                    .send(AppEvent::TaskList(TaskListEvent::ShowTaskDescription)),
                KeyCode::Up => self
                    .events
                    .send(AppEvent::TaskList(TaskListEvent::SelectPrevious)),
                KeyCode::Down => self
                    .events
                    .send(AppEvent::TaskList(TaskListEvent::SelectNext)),
                _ => (),
            }
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    fn focus_next(&mut self) {
        self.focused_widget = match self.focused_widget {
            FocusedWidget::TopBar => FocusedWidget::TaskList,
            FocusedWidget::TaskList => FocusedWidget::TaskDescription,
            FocusedWidget::TaskDescription => FocusedWidget::Gant,
            FocusedWidget::Gant => FocusedWidget::Help,
            FocusedWidget::Help => FocusedWidget::TopBar,
        };
    }

    fn focus_previous(&mut self) {
        self.focused_widget = match self.focused_widget {
            FocusedWidget::TopBar => FocusedWidget::Help,
            FocusedWidget::TaskList => FocusedWidget::TopBar,
            FocusedWidget::TaskDescription => FocusedWidget::TaskList,
            FocusedWidget::Gant => FocusedWidget::TaskDescription,
            FocusedWidget::Help => FocusedWidget::Gant,
        };
    }

    fn handle_task_list_event(&mut self, task_list_event: TaskListEvent) {
        match task_list_event {
            TaskListEvent::SelectNext => self.task_list.state.select_next(),
            TaskListEvent::SelectPrevious => self.task_list.state.select_previous(),
            TaskListEvent::Deselect => self.task_list.state.select(None),
            TaskListEvent::ShowTaskDescription => self.task_list.update_selected_task(),
        }
    }
}
