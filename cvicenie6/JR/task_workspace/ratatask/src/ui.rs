use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Paragraph, Row, Table, TableState, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        let mut tasks = self
            .tm
            .get_tasks()
            .iter()
            .map(|t| format!("{} {} {}", t.id, t.name, t.description))
            .collect::<Vec<_>>()
            .join("\n");

        let start = self.scroll_offset;
        let end = (start + 10).min(self.tm.get_tasks().len());

        let rows = self
            .tm
            .get_tasks()
            .iter()
            .enumerate()
            .skip(start)
            .take(end - start)
            .map(|(i, t)| {
                let mut row = Row::new(vec![
                    Cell::from(t.id.to_string()),
                    Cell::from(t.name.clone()),
                    Cell::from(t.description.clone()),
                ]);

                if i == self.selected {
                    row = row.style(Style::default().bg(Color::Blue).fg(Color::White));
                }
                row
            });

        let mut state = TableState::default();
        state.select(Some(self.selected));
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(2),
                Constraint::Percentage(49),
                Constraint::Percentage(49),
            ],
        )
        .header(Row::new(vec![
            Cell::from("ID").bold(),
            Cell::from("Name").bold(),
            Cell::from("Description").bold(),
        ]))
        .block(
            Block::bordered()
                .title("Tasks")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded),
        )
        .fg(Color::Cyan)
        .bg(Color::Black)
        .column_spacing(2)
        .highlight_symbol(">> ");
        let text = format!(
            "This is your To-Do Task Manager.\n\
                Tasks:\n{}",
            tasks
        );

        let info_text = if let Some(task) = self.tm.get_tasks().get(self.selected) {
            task.get_print_string()
        } else {
            "No task selected".to_string()
        };
        let paragraph = Paragraph::new(info_text)
            .block(
                Block::bordered()
                    .title("Task Info")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded),
            )
            .fg(Color::Cyan)
            .bg(Color::Black)
            .left_aligned();

        let footer_text = Line::from(vec![
            Span::styled("↑↓ Move  ", Style::default().fg(Color::Yellow)),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": Add Task  "),
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::raw(": Quit"),
        ]);

        let footer = Paragraph::new(footer_text)
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .title("Help")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);
        table.render(layout[0], buf);
        paragraph.render(layout[1], buf);
        footer.render(layout[2], buf);
    }
}
