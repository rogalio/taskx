use anyhow::Result;
use std::error;

use crate::models::{Task, TaskStatus, TaskPriority};
use crate::storage::storage_manager;

pub type AppResult<T> = Result<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Kanban,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTab {
    Tasks,
    Calendar,
    Statistics,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub mode: AppMode,
    pub input_mode: InputMode,
    pub current_tab: AppTab,
    pub selected_task_index: Option<usize>,
    pub new_task_input: String,
    pub status_filter: Option<TaskStatus>,
    pub priority_filter: Option<TaskPriority>,
    pub search_query: String,
    pub filter_active: bool,
    pub show_help: bool,
    pub show_task_details: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            mode: AppMode::Kanban,
            input_mode: InputMode::Normal,
            current_tab: AppTab::Tasks,
            selected_task_index: None,
            new_task_input: String::new(),
            status_filter: None,
            priority_filter: None,
            search_query: String::new(),
            filter_active: false,
            show_help: false,
            show_task_details: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_tasks(&mut self) -> Result<()> {
        self.tasks = storage_manager::load_tasks()?;
        if self.tasks.is_empty() {
            self.add_sample_tasks();
        }
        Ok(())
    }

    pub fn add_sample_tasks(&mut self) {
        let sample_tasks = vec![
            Task::new("Implement task creation", "Add ability to create new tasks", TaskStatus::Todo, TaskPriority::High),
            Task::new("Design kanban view", "Create a beautiful kanban board UI", TaskStatus::InProgress, TaskPriority::Critical),
            Task::new("Add keyboard shortcuts", "Implement intuitive keyboard navigation", TaskStatus::Todo, TaskPriority::Medium),
            Task::new("Create task filtering", "Allow filtering by status and priority", TaskStatus::Todo, TaskPriority::Low),
            Task::new("Implement persistence", "Save tasks to disk", TaskStatus::InProgress, TaskPriority::High),
            Task::new("Add due dates", "Implement due date field and sorting", TaskStatus::Todo, TaskPriority::Medium),
            Task::new("Create list view", "Implement alternative list view", TaskStatus::Done, TaskPriority::Medium),
            Task::new("Add task search", "Implement text search functionality", TaskStatus::Done, TaskPriority::Low),
            Task::new("Design statistics view", "Create charts for task progress", TaskStatus::Todo, TaskPriority::Medium),
            Task::new("Write documentation", "Document usage and shortcuts", TaskStatus::Todo, TaskPriority::Low),
        ];

        self.tasks.extend(sample_tasks);
    }

    pub fn save_tasks(&self) -> Result<()> {
        storage_manager::save_tasks(&self.tasks)?;
        Ok(())
    }

    pub fn add_task(&mut self, title: String, description: String, status: TaskStatus, priority: TaskPriority) {
        let task = Task::new(&title, &description, status, priority);
        self.tasks.push(task);
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            AppMode::Normal => AppMode::Kanban,
            AppMode::Kanban => AppMode::List,
            AppMode::List => AppMode::Normal,
        };
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Tasks => AppTab::Calendar,
            AppTab::Calendar => AppTab::Statistics,
            AppTab::Statistics => AppTab::Tasks,
        };
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Tasks => AppTab::Statistics,
            AppTab::Calendar => AppTab::Tasks,
            AppTab::Statistics => AppTab::Calendar,
        };
    }

    pub fn toggle_input_mode(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Normal => InputMode::Editing,
            InputMode::Editing => InputMode::Normal,
        };
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn filtered_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| {
                if !self.filter_active {
                    return true;
                }

                let status_match = self.status_filter
                    .map_or(true, |status| task.status == status);

                let priority_match = self.priority_filter
                    .map_or(true, |priority| task.priority == priority);

                let search_match = self.search_query.is_empty() ||
                    task.title.to_lowercase().contains(&self.search_query.to_lowercase()) ||
                    task.description.to_lowercase().contains(&self.search_query.to_lowercase());

                status_match && priority_match && search_match
            })
            .collect()
    }

    pub fn select_next_task(&mut self) {
        let tasks = self.filtered_tasks();
        if tasks.is_empty() {
            self.selected_task_index = None;
            return;
        }

        if let Some(current_idx) = self.selected_task_index {
            if current_idx >= tasks.len() - 1 {
                self.selected_task_index = Some(0);
                return;
            }

            // Only in Kanban mode, we want to avoid jumping between columns unexpectedly
            if self.mode == AppMode::Kanban && current_idx < tasks.len() - 1 {
                let current_status = tasks[current_idx].status;
                let mut next_idx = current_idx + 1;

                // If next task would be in a different column and we're not at the end of current column,
                // find the next task in the same column
                if tasks[next_idx].status != current_status {
                    // Check if there are more tasks in this column
                    let same_column_tasks: Vec<usize> = tasks.iter()
                        .enumerate()
                        .filter(|(_, t)| t.status == current_status)
                        .map(|(i, _)| i)
                        .collect();

                    let current_pos_in_column = same_column_tasks.iter().position(|&i| i == current_idx);

                    if let Some(pos) = current_pos_in_column {
                        // If we're not at the end of this column, stay in the same column
                        if pos < same_column_tasks.len() - 1 {
                            next_idx = same_column_tasks[pos + 1];
                        }
                    }
                }

                self.selected_task_index = Some(next_idx);
            } else {
                // In other modes or when we want to switch columns, simply go to the next task
                self.selected_task_index = Some(current_idx + 1);
            }
        } else {
            self.selected_task_index = Some(0);
        }
    }

    pub fn select_previous_task(&mut self) {
        let tasks = self.filtered_tasks();
        if tasks.is_empty() {
            self.selected_task_index = None;
            return;
        }

        if let Some(current_idx) = self.selected_task_index {
            if current_idx == 0 {
                self.selected_task_index = Some(tasks.len() - 1);
                return;
            }

            // Only in Kanban mode, we want to avoid jumping between columns unexpectedly
            if self.mode == AppMode::Kanban && current_idx > 0 {
                let current_status = tasks[current_idx].status;
                let mut prev_idx = current_idx - 1;

                // If previous task would be in a different column and we're not at the beginning of current column,
                // find the previous task in the same column
                if tasks[prev_idx].status != current_status {
                    // Check if there are more tasks in this column
                    let same_column_tasks: Vec<usize> = tasks.iter()
                        .enumerate()
                        .filter(|(_, t)| t.status == current_status)
                        .map(|(i, _)| i)
                        .collect();

                    let current_pos_in_column = same_column_tasks.iter().position(|&i| i == current_idx);

                    if let Some(pos) = current_pos_in_column {
                        // If we're not at the beginning of this column, stay in the same column
                        if pos > 0 {
                            prev_idx = same_column_tasks[pos - 1];
                        }
                    }
                }

                self.selected_task_index = Some(prev_idx);
            } else {
                // In other modes or when we want to switch columns, simply go to the previous task
                self.selected_task_index = Some(current_idx - 1);
            }
        } else {
            self.selected_task_index = Some(tasks.len() - 1);
        }
    }

    pub fn move_task_status(&mut self) -> Result<()> {
        if let Some(index) = self.selected_task_index {
            let tasks = self.filtered_tasks();
            if let Some(task_ref) = tasks.get(index) {
                // Find the actual index in the main tasks vector
                if let Some(actual_index) = self.tasks.iter().position(|t| t.id == task_ref.id) {
                    let next_status = match self.tasks[actual_index].status {
                        TaskStatus::Todo => TaskStatus::InProgress,
                        TaskStatus::InProgress => TaskStatus::Done,
                        TaskStatus::Done => TaskStatus::Todo,
                    };
                    self.tasks[actual_index].status = next_status;
                    self.save_tasks()?;
                }
            }
        }
        Ok(())
    }

    // Navigates to the next column in Kanban view
    pub fn next_column(&mut self) {
        if self.mode != AppMode::Kanban {
            return;
        }

        let tasks = self.filtered_tasks();
        if tasks.is_empty() {
            return;
        }

        // Determine which column to navigate to
        let current_status = if let Some(idx) = self.selected_task_index {
            if idx < tasks.len() {
                tasks[idx].status
            } else {
                TaskStatus::Todo
            }
        } else {
            TaskStatus::Todo
        };

        // Find the next column status
        let next_status = match current_status {
            TaskStatus::Todo => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::Done,
            TaskStatus::Done => TaskStatus::Todo,
        };

        // Find the first task in the next column
        let next_column_tasks: Vec<usize> = tasks.iter()
            .enumerate()
            .filter(|(_, t)| t.status == next_status)
            .map(|(i, _)| i)
            .collect();

        if !next_column_tasks.is_empty() {
            self.selected_task_index = Some(next_column_tasks[0]);
        }
    }

    // Navigates to the previous column in Kanban view
    pub fn previous_column(&mut self) {
        if self.mode != AppMode::Kanban {
            return;
        }

        let tasks = self.filtered_tasks();
        if tasks.is_empty() {
            return;
        }

        // Determine which column to navigate to
        let current_status = if let Some(idx) = self.selected_task_index {
            if idx < tasks.len() {
                tasks[idx].status
            } else {
                TaskStatus::Todo
            }
        } else {
            TaskStatus::Todo
        };

        // Find the previous column status
        let prev_status = match current_status {
            TaskStatus::Todo => TaskStatus::Done,
            TaskStatus::InProgress => TaskStatus::Todo,
            TaskStatus::Done => TaskStatus::InProgress,
        };

        // Find the first task in the previous column
        let prev_column_tasks: Vec<usize> = tasks.iter()
            .enumerate()
            .filter(|(_, t)| t.status == prev_status)
            .map(|(i, _)| i)
            .collect();

        if !prev_column_tasks.is_empty() {
            self.selected_task_index = Some(prev_column_tasks[0]);
        }
    }
}