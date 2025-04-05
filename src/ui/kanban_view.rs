use itertools::Itertools;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::{Task, TaskStatus};
use crate::utils;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    let tasks = app.filtered_tasks();

    // Group tasks by status
    let todo_tasks: Vec<&Task> = tasks.iter()
        .filter(|&&t| t.status == TaskStatus::Todo)
        .cloned()
        .collect();

    let in_progress_tasks: Vec<&Task> = tasks.iter()
        .filter(|&&t| t.status == TaskStatus::InProgress)
        .cloned()
        .collect();

    let done_tasks: Vec<&Task> = tasks.iter()
        .filter(|&&t| t.status == TaskStatus::Done)
        .cloned()
        .collect();

    // Render columns
    render_column(frame, app, chunks[0], "TODO", &todo_tasks, TaskStatus::Todo);
    render_column(frame, app, chunks[1], "IN PROGRESS", &in_progress_tasks, TaskStatus::InProgress);
    render_column(frame, app, chunks[2], "DONE", &done_tasks, TaskStatus::Done);
}

fn render_column(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    title: &str,
    tasks: &[&Task],
    status: TaskStatus,
) {
    let color = match status {
        TaskStatus::Todo => Color::Blue,
        TaskStatus::InProgress => Color::Green,
        TaskStatus::Done => Color::DarkGray,
    };

    let block = Block::default()
        .title(format!(" {} ({}) ", title, tasks.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    if tasks.is_empty() {
        let text = Text::from(Line::from(vec![
            Span::styled("No tasks", Style::default().fg(Color::DarkGray)),
        ]));

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
        return;
    }

    let selected_index = if app.selected_task_index.is_some() {
        let all_tasks = app.filtered_tasks();
        if let Some(idx) = app.selected_task_index {
            if idx < all_tasks.len() {
                let selected_task = all_tasks[idx];
                if selected_task.status == status {
                    tasks.iter().position(|&t| t.id == selected_task.id)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let items: Vec<ListItem> = tasks
        .iter()
        .map(|&task| {
            let priority_color = utils::parse_color(task.priority_color());

            // Task title with priority indicator
            let header = Line::from(vec![
                Span::styled(
                    format!("[{}] ", task.priority),
                    Style::default().fg(priority_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(&task.title),
            ]);

            let priority = Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    utils::truncate_string(&task.description, 50),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(vec![header, priority])
                .style(Style::default())
        })
        .collect();

    let tasks_list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(selected_index);

    frame.render_stateful_widget(tasks_list, area, &mut state);
}