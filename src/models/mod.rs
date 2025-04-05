use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumIter)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumIter, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(title: &str, description: &str, status: TaskStatus, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status,
            priority,
            created_at: Utc::now(),
            due_date: None,
            tags: Vec::new(),
        }
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            due < Utc::now() && self.status != TaskStatus::Done
        } else {
            false
        }
    }

    pub fn priority_color(&self) -> &str {
        match self.priority {
            TaskPriority::Low => "#72B7F0", // Light blue
            TaskPriority::Medium => "#70C666", // Green
            TaskPriority::High => "#FF9E3B", // Orange
            TaskPriority::Critical => "#FF6369", // Red
        }
    }

    pub fn status_color(&self) -> &str {
        match self.status {
            TaskStatus::Todo => "#72B7F0",     // Light blue
            TaskStatus::InProgress => "#70C666", // Green
            TaskStatus::Done => "#6E7781",       // Gray
        }
    }
}