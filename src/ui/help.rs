use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, _app: &App) {
    let size = frame.size();
    let popup_area = centered_rect(80, 80, size);

    // Clear the area where the popup will be rendered
    frame.render_widget(Clear, popup_area);

    // Create help popup
    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    // Create the help text with different sections
    let mut text_spans = Vec::new();

    // Title
    text_spans.push(
        Line::from(Span::styled(
            "TaskX - Terminal Task Manager",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
    );
    text_spans.push(Line::from(""));

    // Navigation section
    text_spans.push(
        Line::from(Span::styled(
            "Navigation:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))
    );
    add_help_item(&mut text_spans, "↑/k", "Move selection up");
    add_help_item(&mut text_spans, "↓/j", "Move selection down");
    add_help_item(&mut text_spans, "←/h", "Previous column (Kanban view)");
    add_help_item(&mut text_spans, "→/l", "Next column (Kanban view)");
    add_help_item(&mut text_spans, "Enter", "Toggle task details");
    add_help_item(&mut text_spans, "Tab", "Next tab");
    add_help_item(&mut text_spans, "Shift+Tab", "Previous tab");
    text_spans.push(Line::from(""));

    // Task management section
    text_spans.push(
        Line::from(Span::styled(
            "Task Management:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))
    );
    add_help_item(&mut text_spans, "n", "Create new task");
    add_help_item(&mut text_spans, "Space", "Move task to next status");
    add_help_item(&mut text_spans, "e/i", "Edit task (not implemented yet)");
    add_help_item(&mut text_spans, "d", "Delete task (not implemented yet)");
    text_spans.push(Line::from(""));

    // View options section
    text_spans.push(
        Line::from(Span::styled(
            "View Options:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))
    );
    add_help_item(&mut text_spans, "v", "Toggle view mode (Kanban/List/Detail)");
    add_help_item(&mut text_spans, "f", "Toggle filter mode");
    add_help_item(&mut text_spans, "1", "Filter Todo tasks");
    add_help_item(&mut text_spans, "2", "Filter In Progress tasks");
    add_help_item(&mut text_spans, "3", "Filter Done tasks");
    text_spans.push(Line::from(""));

    // General section
    text_spans.push(
        Line::from(Span::styled(
            "General:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ))
    );
    add_help_item(&mut text_spans, "?", "Toggle help");
    add_help_item(&mut text_spans, "q", "Quit");
    text_spans.push(Line::from(""));

    // Footer
    text_spans.push(
        Line::from(Span::styled(
            "Press any key to close this help screen",
            Style::default().fg(Color::Yellow),
        ))
    );

    let text = Text::from(text_spans);

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, popup_area);
}

fn add_help_item(spans: &mut Vec<Line<'static>>, key: &str, description: &str) {
    spans.push(Line::from(vec![
        Span::styled(format!("  {:<12}", key), Style::default().fg(Color::Cyan)),
        Span::raw(description.to_string()),
    ]));
}

/// Helper function to create a centered rect using up certain percentage of the available rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}