use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

use crate::app::{App, AppResult, InputMode};
use crate::models::{TaskStatus, TaskPriority};

pub fn event_handler(app: &mut App) -> AppResult<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            return match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::Editing => handle_editing_mode(app, key),
            };
        }
    }
    Ok(false)
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> AppResult<bool> {
    // If task details is open, handle Escape key differently
    if app.show_task_details {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                app.show_task_details = false;
                return Ok(false);
            },
            _ => return Ok(false), // Ignore other keys when task details is open
        }
    }

    match key.code {
        // Quit application
        KeyCode::Char('q') => return Ok(true),

        // Help
        KeyCode::Char('?') => app.toggle_help(),

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => app.select_next_task(),
        KeyCode::Up | KeyCode::Char('k') => app.select_previous_task(),
        KeyCode::Right | KeyCode::Char('l') => app.next_column(),
        KeyCode::Left | KeyCode::Char('h') => app.previous_column(),

        // Toggle task details
        KeyCode::Enter => {
            app.show_task_details = !app.show_task_details;
        },

        // Tab navigation
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.previous_tab(),

        // Toggle view mode
        KeyCode::Char('v') => app.toggle_mode(),

        // Enter edit mode
        KeyCode::Char('e') | KeyCode::Char('i') => {
            app.toggle_input_mode();
        }

        // Move task status
        KeyCode::Char(' ') => {
            app.move_task_status()?;
        }

        // New task
        KeyCode::Char('n') => {
            app.new_task_input.clear();
            app.toggle_input_mode();
        }

        // Toggle filter
        KeyCode::Char('f') => {
            app.filter_active = !app.filter_active;
        }

        // Filter by status
        KeyCode::Char('1') => {
            app.status_filter = Some(TaskStatus::Todo);
            app.filter_active = true;
        }
        KeyCode::Char('2') => {
            app.status_filter = Some(TaskStatus::InProgress);
            app.filter_active = true;
        }
        KeyCode::Char('3') => {
            app.status_filter = Some(TaskStatus::Done);
            app.filter_active = true;
        }

        _ => {}
    }
    Ok(false)
}

fn handle_editing_mode(app: &mut App, key: KeyEvent) -> AppResult<bool> {
    match key.code {
        // Exit edit mode
        KeyCode::Esc => {
            app.toggle_input_mode();
        }

        // Submit new task
        KeyCode::Enter => {
            if !app.new_task_input.is_empty() {
                app.add_task(
                    app.new_task_input.clone(),
                    "".to_string(),
                    TaskStatus::Todo,
                    TaskPriority::Medium,
                );
                app.save_tasks()?;
                app.new_task_input.clear();
                app.toggle_input_mode();
            }
        }

        // Edit text
        KeyCode::Char(c) => {
            app.new_task_input.push(c);
        }

        KeyCode::Backspace => {
            app.new_task_input.pop();
        }

        _ => {}
    }
    Ok(false)
}