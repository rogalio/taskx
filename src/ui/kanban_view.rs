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
use crate::models::{Task, TaskStatus, TaskPriority};
use crate::utils;
use crate::ui::colors;

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

    // Render columns with different colors
    render_column(frame, app, chunks[0], "TODO", &todo_tasks, TaskStatus::Todo, Color::Blue);
    render_column(frame, app, chunks[1], "IN PROGRESS", &in_progress_tasks, TaskStatus::InProgress, Color::Green);
    render_column(frame, app, chunks[2], "DONE", &done_tasks, TaskStatus::Done, Color::Gray);
}

// Function to get priority weight for sorting (higher priority = higher weight)
fn get_priority_weight(task: &&Task) -> u8 {
    match task.priority {
        TaskPriority::Critical => 4,
        TaskPriority::High => 3,
        TaskPriority::Medium => 2,
        TaskPriority::Low => 1,
    }
}

fn render_column(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    title: &str,
    tasks: &[&Task],
    status: TaskStatus,
    color: Color,
) {
    // Create title with count in parentheses
    let title = format!(" {} ({}) ", title, tasks.len());

    // Define column block with custom border color
    let column_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    // Adjust area for inner content
    let inner_area = column_block.inner(area);

    // Render the column block first
    frame.render_widget(column_block, area);

    if tasks.is_empty() {
        let text = Text::from(Line::from(vec![
            Span::styled("No tasks", Style::default().fg(Color::DarkGray)),
        ]));

        let paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, inner_area);
        return;
    }

    // Sort tasks by priority (highest priority first)
    let sorted_tasks: Vec<&Task> = tasks.iter()
        .cloned()
        .sorted_by(|a, b| get_priority_weight(&b).cmp(&get_priority_weight(&a)))
        .collect();

    let selected_index = if app.selected_task_index.is_some() {
        let all_tasks = app.filtered_tasks();
        if let Some(idx) = app.selected_task_index {
            if idx < all_tasks.len() {
                let selected_task = all_tasks[idx];
                if selected_task.status == status {
                    sorted_tasks.iter().position(|&t| t.id == selected_task.id)
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

    // Create individual task cards with borders
    let task_items: Vec<ListItem> = sorted_tasks
        .iter()
        .map(|&task| {
            // Get priority color for badge
            let priority_color = match task.priority.to_string().as_str() {
                "Critical" => Color::Rgb(239, 68, 68),    // Red
                "High" => Color::Rgb(251, 191, 36),       // Orange
                "Medium" => Color::Rgb(16, 185, 129),     // Green
                "Low" => Color::Rgb(59, 130, 246),        // Blue
                _ => Color::DarkGray,
            };

            // Get label based on priority
            let label = match task.priority.to_string().as_str() {
                "Critical" => " Bug ",
                "High" => " Feature ",
                "Medium" => " Task ",
                _ => " Other ",
            };

            // Create single line with priority badge, title, and label badge
            let task_line = Line::from(vec![
                // Priority badge first
                Span::styled(
                    format!(" {} ", task.priority),
                    Style::default()
                        .bg(priority_color)
                        .fg(Color::Black)
                ),
                Span::raw(" "),
                // Title in the middle
                Span::styled(
                    task.title.clone(),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                // Label badge last
                Span::styled(
                    label,
                    Style::default()
                        .bg(Color::Rgb(60, 60, 60))
                        .fg(Color::White),
                ),
            ]);

            // Create task card with border and padding
            let task_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            // Create list item with block and content
            // Add empty lines for spacing between cards
            ListItem::new(vec![
                task_line,
            ])
            .style(Style::default())
        })
        .intersperse(
            // Add an empty ListItem between each task for spacing
            ListItem::new(vec![Line::from("")])
                .style(Style::default())
        )
        .collect();

    // Create list of task cards
    let tasks_list = List::new(task_items)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow) // Just highlight text in yellow instead of background
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("");

    let mut state = ListState::default();
    state.select(selected_index.map(|idx| idx * 2)); // Adjust for interspersed empty items

    frame.render_stateful_widget(tasks_list, inner_area, &mut state);
}