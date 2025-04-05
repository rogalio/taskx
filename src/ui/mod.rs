mod kanban_view;
mod list_view;
mod task_detail;
mod help;
mod tabs;
mod colors;

use itertools::Itertools;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::{App, AppMode, AppTab, InputMode};
use crate::models::{TaskStatus, TaskPriority};
use crate::utils;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tab bar
            Constraint::Min(0),     // Main content
            Constraint::Length(2),  // Input bar
        ])
        .split(frame.size());

    render_tabs(frame, app, chunks[0]);

    match app.current_tab {
        AppTab::Tasks => match app.mode {
            AppMode::Kanban => kanban_view::render(frame, app, chunks[1]),
            AppMode::List => list_view::render(frame, app, chunks[1]),
            AppMode::Normal => {
                if let Some(index) = app.selected_task_index {
                    let tasks = app.filtered_tasks();
                    if let Some(task) = tasks.get(index) {
                        task_detail::render(frame, app, chunks[1], task);
                    }
                }
            }
        },
        AppTab::Calendar => render_calendar(frame, app, chunks[1]),
        AppTab::Statistics => render_statistics(frame, app, chunks[1]),
    }

    render_input_bar(frame, app, chunks[2]);

    // Handle task details modal
    if app.show_task_details {
        if let Some(index) = app.selected_task_index {
            let tasks = app.filtered_tasks();
            if let Some(task) = tasks.get(index) {
                task_detail::render(frame, app, chunks[1], task);
            }
        }
    }

    if app.show_help {
        help::render(frame, app);
    }
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles = vec![
        Line::from(" Tasks "),
        Line::from(" Calendar "),
        Line::from(" Statistics "),
    ];

    let selected_tab = app.current_tab as usize;

    let tab_widget = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .select(selected_tab)
        .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    frame.render_widget(tab_widget, area);
}

fn render_calendar(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Calendar View ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let text = Text::from(Line::from(vec![
        Span::styled("Calendar view ", Style::default().fg(Color::Yellow)),
        Span::raw("coming soon!"),
    ]));

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_statistics(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Statistics ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    // Collect task statistics
    let total = app.tasks.len();
    let todo = app.tasks.iter().filter(|t| t.status == TaskStatus::Todo).count();
    let in_progress = app.tasks.iter().filter(|t| t.status == TaskStatus::InProgress).count();
    let done = app.tasks.iter().filter(|t| t.status == TaskStatus::Done).count();

    let mut lines = Vec::new();

    lines.push(Line::from(
        Span::styled("Task Statistics", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    ));

    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::raw("Total Tasks: "),
        Span::styled(total.to_string(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
    ]));

    lines.push(Line::from(vec![
        Span::raw("Todo: "),
        Span::styled(todo.to_string(), Style::default().fg(Color::Blue)),
        Span::raw("  In Progress: "),
        Span::styled(in_progress.to_string(), Style::default().fg(Color::Green)),
        Span::raw("  Done: "),
        Span::styled(done.to_string(), Style::default().fg(Color::Gray)),
    ]));

    lines.push(Line::from(""));

    lines.push(Line::from(
        Span::styled("Priority Breakdown", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    ));

    lines.push(Line::from(vec![
        Span::raw("Critical: "),
        Span::styled(
            app.tasks.iter().filter(|t| t.priority == TaskPriority::Critical).count().to_string(),
            Style::default().fg(Color::Red)
        ),
    ]));

    lines.push(Line::from(vec![
        Span::raw("High: "),
        Span::styled(
            app.tasks.iter().filter(|t| t.priority == TaskPriority::High).count().to_string(),
            Style::default().fg(Color::LightRed)
        ),
    ]));

    lines.push(Line::from(vec![
        Span::raw("Medium: "),
        Span::styled(
            app.tasks.iter().filter(|t| t.priority == TaskPriority::Medium).count().to_string(),
            Style::default().fg(Color::Yellow)
        ),
    ]));

    lines.push(Line::from(vec![
        Span::raw("Low: "),
        Span::styled(
            app.tasks.iter().filter(|t| t.priority == TaskPriority::Low).count().to_string(),
            Style::default().fg(Color::Blue)
        ),
    ]));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_input_bar(frame: &mut Frame, app: &App, area: Rect) {
    let text = match app.input_mode {
        InputMode::Normal => {
            Text::from(Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Blue)),
                Span::styled("?", Style::default().fg(Color::Yellow)),
                Span::styled(" for help", Style::default().fg(Color::Blue)),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::styled(" to quit", Style::default().fg(Color::Blue)),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled("n", Style::default().fg(Color::Yellow)),
                Span::styled(" new task", Style::default().fg(Color::Blue)),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled("v", Style::default().fg(Color::Yellow)),
                Span::styled(" toggle view", Style::default().fg(Color::Blue)),
            ]))
        },
        InputMode::Editing => {
            Text::from(Line::from(vec![
                Span::styled("New task: ", Style::default().fg(Color::Yellow)),
                Span::styled(&app.new_task_input, Style::default().fg(Color::White)),
                Span::styled(" (ESC to cancel, ENTER to save)", Style::default().fg(Color::DarkGray)),
            ]))
        },
    };

    let input_block = Paragraph::new(text)
        .block(Block::default().borders(Borders::TOP))
        .wrap(Wrap { trim: true });

    frame.render_widget(input_block, area);
}