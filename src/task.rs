use std::fs;

use serde::{Deserialize, Serialize};

use crate::fileio::{get_task_file, read_task_file};
use crate::operation::{ListArgs, UpdateArgs};
use crate::{
    operation::{AddArgs, ReadArgs},
    task,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

pub fn list(ListArgs { tags }: ListArgs) {
    let mut task_list = String::new();

    let mut tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    let total_tasks = tasks.len();

    if let Some(tags) = tags {
        tasks.retain(|task| {
            if let Some(task_tags) = &task.tags {
                task_tags.iter().any(|t| tags.contains(t))
            } else {
                false
            }
        });
    }
    
    for task in &tasks {
        task_list.push_str(&format!("{}: {}, Tags: {}\n", task.id, task.name, task.tags.as_deref().unwrap_or(&[]).join(", ")));
    }
    if tasks.is_empty() {
        println!("No tasks found matching the criteria. Total tasks: {}", total_tasks);
    } else {
        println!("Showing {} tasks of {}:\n{}", tasks.len(), total_tasks, task_list);
    }
}

pub fn add(
    AddArgs {
        id,
        name,
        description,
        tags,
    }: AddArgs,
) {
    let mut tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    if tasks.iter().any(|t| t.id == id) {
        eprintln!("Task with ID '{}' already exists.", id);
        return;
    }

    let new_task = task::Task {
        id,
        name,
        description,
        tags,
    };

    tasks.push(new_task);

    let task_file = get_task_file();
    fs::write(task_file, serde_json::to_string_pretty(&tasks).unwrap())
        .expect("Failed to write tasks file");

    println!("Task added successfully.");
}

pub fn show(ReadArgs { id }: ReadArgs) {
    let tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    if let Some(task) = tasks.iter().find(|t| t.id == id) {
        println!(
            "ID: {}\nName: {}\nDescription: {}\nTags: {}",
            task.id,
            task.name,
            task.description.as_deref().unwrap_or("None"),
            task.tags.as_deref().unwrap_or(&[]).join(", ")
        );
    } else {
        eprintln!("Task with ID '{}' not found.", id);
    }
}

pub fn remove(ReadArgs { id }: ReadArgs) {
    println!("Are you sure you want to remove the task '{}'?", id);
    if !crate::util::get_yes_no() {
        println!("Task removal cancelled.");
        return;
    }

    let mut tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        fs::write(get_task_file(), serde_json::to_string_pretty(&tasks).unwrap())
            .expect("Failed to write tasks file");
        println!("Task '{}' removed successfully.", id);
    } else {
        eprintln!("Task '{}' not found.", id);
    }
}

pub fn update(
    UpdateArgs {
        id,
        name,
        description,
        tags,
    }: UpdateArgs,
) {
    let mut tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        if let Some(new_name) = name {
            task.name = new_name;
        }
        if let Some(new_description) = description {
            task.description = Some(new_description);
        }
        if let Some(new_tags) = tags {
            task.tags = Some(new_tags);
        }

        fs::write(get_task_file(), serde_json::to_string_pretty(&tasks).unwrap())
            .expect("Failed to write tasks file");
        println!("Task '{}' updated successfully.", id);
    } else {
        eprintln!("Task with ID '{}' not found.", id);
    }
}

// intended for .bashrc or similar
pub fn summary() {
    let tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    // Exit quietly if there are no tasks
    // to avoid unneeded terminal clutter
    if tasks.is_empty() {
        return;
    }

    println!("[TaskTrack] Tasks Remaining: {}", tasks.len());
}
