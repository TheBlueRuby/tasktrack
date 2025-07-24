use std::{
    env, fs,
    path::PathBuf,
};

use crate::{operation::{AddArgs, ReadArgs}, task};

//TODO: custom task and config file
fn get_task_file() -> String {
    let task_file: PathBuf = env::home_dir()
        .unwrap_or_else(|| {
            eprintln!("Could not determine home directory.");
            PathBuf::from(".")
        })
        .join(".tasktrack/tasks.json");

    if !task_file.exists() {
        // Create the directory if it doesn't exist
        if let Some(parent) = task_file.parent() {
            fs::create_dir_all(parent).expect("Failed to create task directory");
        }
        // Create an empty tasks file
        fs::write(&task_file, "[]").expect("Failed to create tasks file");
    }
    task_file.to_str().unwrap_or_default().to_string()
}

fn read_task_file() -> String {
    let task_file = get_task_file();
    match fs::read_to_string(&task_file) {
        Ok(filestr) => filestr,
        Err(err) => {
            eprintln!("Error reading task file: {}", err);
            return "[]".to_string(); // Return empty JSON array if file read fails
        }
    }
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
    fs::write(task_file, serde_json::to_string(&tasks).unwrap())
        .expect("Failed to write tasks file");

    println!("Task added successfully.");
}

pub fn show(ReadArgs { id }: ReadArgs) {
    let tasks: Vec<task::Task> =
        serde_json::from_str(&read_task_file()).expect("Failed to parse tasks file");

    if let Some(task) = tasks.iter().find(|t| t.id == id) {
        println!("ID: {}\nName: {}\nDescription: {}", task.id, task.name, task.description.as_deref().unwrap_or("None"));
    } else {
        eprintln!("Task with ID '{}' not found.", id);
    }
}
