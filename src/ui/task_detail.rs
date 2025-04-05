use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use chrono::{DateTime, Local, Utc};
use crate::app::App;
use crate::models::Task;
use crate::utils;

pub fn render(frame: &mut Frame, app: &App, area: Rect, task: &Task) {
    // Only show modal if the flag is set
    if !app.show_task_details {
        return;
    }

    let size = frame.size();
    let popup_area = centered_rect(80, 80, size);

    // Clear the area where the popup will be rendered
    frame.render_widget(Clear, popup_area);

    // Create a layout for the task details
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(2),  // Creation date
            Constraint::Length(2),  // Status
            Constraint::Length(2),  // Priority
            Constraint::Min(5),     // Description
            Constraint::Length(1),  // Empty space
            Constraint::Length(1),  // Footer
        ])
        .split(popup_area);

    // Create borders around the entire popup
    let block = Block::default()
        .title(format!(" Task Details ({}) ", task.id.chars().take(8).collect::<String>()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, popup_area);

    // Title section
    let title_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Title: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(&task.title),
        ]),
    ]);
    let title_paragraph = Paragraph::new(title_text);
    frame.render_widget(title_paragraph, chunks[0]);

    // Created at
    let created_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Created: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(task.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ]),
    ]);
    let created_paragraph = Paragraph::new(created_text);
    frame.render_widget(created_paragraph, chunks[1]);

    // Status with colored indicator
    let status_color = utils::parse_color(task.status_color());
    let status_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", task.status), Style::default().fg(status_color)),
        ]),
    ]);
    let status_paragraph = Paragraph::new(status_text);
    frame.render_widget(status_paragraph, chunks[2]);

    // Priority with colored indicator
    let priority_color = utils::parse_color(task.priority_color());
    let priority_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Priority: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", task.priority), Style::default().fg(priority_color)),
        ]),
    ]);
    let priority_paragraph = Paragraph::new(priority_text);
    frame.render_widget(priority_paragraph, chunks[3]);

    // Description with wrapping
    let description_block = Block::default()
        .title(" Description ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let description_text = if task.description.is_empty() {
        Text::from(Line::from(vec![
            Span::styled("No description provided", Style::default().fg(Color::DarkGray)),
        ]))
    } else {
        Text::from(task.description.as_str())
    };

    let description_paragraph = Paragraph::new(description_text)
        .block(description_block)
        .wrap(Wrap { trim: true });

    frame.render_widget(description_paragraph, chunks[4]);

    // Footer with instructions
    let footer_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::styled(" or ", Style::default().fg(Color::Gray)),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::styled(" to close", Style::default().fg(Color::Gray)),
        ]),
    ]);
    let footer_paragraph = Paragraph::new(footer_text);
    frame.render_widget(footer_paragraph, chunks[6]);
}

/// Helper function to create a centered rect using up certain percentage of the available rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn format_date(date: DateTime<Utc>) -> String {
    date.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string()
}