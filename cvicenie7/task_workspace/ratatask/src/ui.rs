use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Bar, Block, BorderType, Borders, List, ListItem, Paragraph, StatefulWidget, Widget,
        canvas::{Canvas, Rectangle},
    },
};
use task_library::task::Task;

use crate::{
    app::{App, FocusedWidget},
    task_list,
};

const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);

impl App {
    pub fn render_top_bar(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TopBar.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if matches!(self.focused_widget, FocusedWidget::TopBar) {
            block = block.border_style(Style::new().bg(Color::DarkGray))
        }

        Paragraph::new(Text::from(format!(
            "Aktualne pouzivame subor s taskami na: {}",
            self.task_path.to_str().unwrap()
        )))
        .block(block)
        .alignment(Alignment::Center)
        .render(area, buf);
    }

    pub fn render_task_list(&mut self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TaskList.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if matches!(self.focused_widget, FocusedWidget::TaskList) {
            block = block.border_style(Style::new().bg(Color::DarkGray))
        }

        let items: Vec<ListItem> = self
            .task_list
            .task_manager
            .get_tasks()
            .iter()
            .map(|t| ListItem::new(t.format_task()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.task_list.state);
    }

    pub fn render_task_description(&mut self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::TaskDescription.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if matches!(self.focused_widget, FocusedWidget::TaskDescription) {
            block = block.border_style(Style::new().bg(Color::DarkGray))
        }

        let items = match self.task_list.rendered_task.as_ref() {
            Some(t) => t.get_print_string(),
            None => String::new(),
        };

        Paragraph::new(items).block(block).render(area, buf);
    }
    pub fn render_gant(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::Gant.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if matches!(self.focused_widget, FocusedWidget::Gant) {
            block = block.border_style(Style::new().bg(Color::DarkGray))
        }

        let tasks = self.task_list.task_manager.get_tasks();

        if tasks.is_empty() {
            Block::default()
                .title("Gantt Chart")
                .borders(Borders::ALL)
                .render(area, buf);
            return;
        }

        let parsed: Vec<_> = tasks
            .iter()
            .filter_map(|t| {
                let start = t.skutocny_zaciatok?;
                let duration = t.skutocne_trvanie; // seconds (i64)
                Some((t.nazov.clone(), start, duration))
            })
            .collect();

        if parsed.is_empty() {
            block.render(area, buf);
            return;
        }

        let tasks = self.task_list.task_manager.get_tasks();
        let min_date = tasks.iter().map(|t| t.planovany_zaciatok).min().unwrap();
        let max_date = tasks
            .iter()
            .map(|t| t.vypocitaj_planovany_koniec())
            .max()
            .unwrap();
        let total_days = max_date - min_date;

        let canvas = Canvas::default()
            .block(block)
            .x_bounds([0.0, total_days.num_days().max(1) as f64])
            .y_bounds([0.0, tasks.len() as f64])
            .marker(ratatui::symbols::Marker::Block)
            .paint(|context| {
                for (i, task) in tasks.iter().enumerate() {
                    let x = (task.planovany_zaciatok - min_date).num_days() as f64;
                    let y = (tasks.len() - i) as f64;
                    let width = task.planovane_trvanie.num_seconds() as f64 / 86400.0;
                    let height = 0.0;
                    let color = Color::White;

                    context.draw(&Rectangle {
                        x,
                        y,
                        width,
                        height,
                        color,
                    });
                    context.print(0.0, y, format!("#{}", task.id));
                }
            });

        canvas.render(area, buf);
    }

    pub fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .title(FocusedWidget::Help.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        if matches!(self.focused_widget, FocusedWidget::Help) {
            block = block.border_style(Style::new().bg(Color::DarkGray))
        }

        let mut text_lines: Vec<Line<'_>> = vec![
            "Press q to exit".into(),
            "Press Tab to focus next widget. Press Shift+Tab to focus previous widget".into(),
            format!("Currently focused widget: {}", self.focused_widget).into(),
        ];

        if let FocusedWidget::TaskList = self.focused_widget {
            add_task_list_info(&mut text_lines)
        }

        let lines = Text::from(text_lines);
        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

fn add_task_list_info(text_lines: &mut Vec<Line<'_>>) {
    text_lines.push(Line::from("Select task using arrows up ↑ and down ↓. Press Esc to deselect all or Enter to show description"));
}

impl Widget for &mut App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("ratatask")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let areas: [Rect; 4] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(15),
            ])
            .areas(area);

        let inner_area: [Rect; 2] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(areas[1]);

        self.render_top_bar(areas[0], buf);
        self.render_task_list(inner_area[0], buf);
        self.render_task_description(inner_area[1], buf);
        self.render_gant(areas[2], buf);
        self.render_help(areas[3], buf);

        // let text = self.task_manager.format_all_tasks().iter().fold(String::new(), |t1, t2| {
        //     t1 + "\n" + t2
        // });

        // let paragraph = Paragraph::new(text)
        //     .block(block)
        //     .fg(Color::LightBlue)
        //     .bg(Color::Black);

        // paragraph.render(area, buf);
    }
}
