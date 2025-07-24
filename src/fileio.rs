use std::{
    env, fs,
    path::PathBuf,
};

//TODO: custom task and config file
pub fn get_task_file() -> String {
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

pub fn read_task_file() -> String {
    let task_file = get_task_file();
    match fs::read_to_string(&task_file) {
        Ok(filestr) => filestr,
        Err(err) => {
            eprintln!("Error reading task file: {}", err);
            return "[]".to_string(); // Return empty JSON array if file read fails
        }
    }
}

