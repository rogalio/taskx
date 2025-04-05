use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;
use crate::utils;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let tasks = app.filtered_tasks();
    let selected = app.selected_task_index;

    let header_cells = ["ID", "Title", "Status", "Priority"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).style(Style::default().bg(Color::DarkGray));

    let rows = tasks.iter().enumerate().map(|(i, task)| {
        let status_color = utils::parse_color(task.status_color());
        let priority_color = utils::parse_color(task.priority_color());

        let id = Span::raw(format!("#{}", i + 1));
        let title = Span::raw(utils::truncate_string(&task.title, 40));
        let status = Span::styled(format!("{}", task.status), Style::default().fg(status_color));
        let priority = Span::styled(format!("{}", task.priority), Style::default().fg(priority_color));

        let mut row_style = Style::default();
        if selected == Some(i) {
            row_style = row_style.bg(Color::DarkGray);
        }

        let cells = vec![
            Cell::from(id),
            Cell::from(title),
            Cell::from(status),
            Cell::from(priority),
        ];

        Row::new(cells).style(row_style)
    });

    let widths = [
        Constraint::Length(5),
        Constraint::Percentage(60),
        Constraint::Length(14),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title(" Tasks ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(table, area);
}