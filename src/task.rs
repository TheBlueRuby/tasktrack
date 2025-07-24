use std::fs;

use serde::{Deserialize, Serialize};

use crate::fileio::{get_task_file, read_task_file};
use crate::operation::UpdateArgs;
use crate::{
    operation::{AddArgs, ReadArgs},
    task,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub fn list() {
    let mut task_list = String::new();

    let tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    for task in tasks {
        task_list.push_str(&format!("{}: {}\n", task.id, task.name));
    }
    println!("{}", task_list);
}

pub fn add(
    AddArgs {
        id,
        name,
        description,
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
            "ID: {}\nName: {}\nDescription: {}",
            task.id,
            task.name,
            task.description.as_deref().unwrap_or("None")
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
