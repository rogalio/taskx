use anyhow::Result;
use serde_json;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;
use directories::ProjectDirs;

use crate::models::Task;

// TODO: move to config

fn get_data_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "task_manager", "task_manager")
        .ok_or_else(|| anyhow::anyhow!("Could not determine project directories"))?;

    let data_dir = proj_dirs.data_dir();
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }

    Ok(data_dir.to_path_buf())
}

fn get_tasks_file_path() -> Result<PathBuf> {
    let data_dir = get_data_dir()?;
    Ok(data_dir.join("tasks.json"))
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let file_path = get_tasks_file_path()?;
    let json = serde_json::to_string_pretty(tasks)?;

    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>> {
    let file_path = get_tasks_file_path()?;

    // If file doesn't exist, return empty vector
    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tasks: Vec<Task> = serde_json::from_str(&contents)?;
    Ok(tasks)
}