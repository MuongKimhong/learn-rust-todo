use std::path::Path;
use std::fs::File;
use serde_json::to_writer;
use serde_json::from_reader;

use crate::task::Task;

fn file_path_exists(file_path: &String) -> bool {
    return Path::new(file_path).exists();
}

pub fn write_tasks_to_file(all_tasks: &Vec<Task>, file_path: &String) -> std::io::Result<()> {
    if !file_path_exists(file_path) {
        panic!("{file_path} not found");
    }

    let file = File::create(file_path)?;
    to_writer(file, &all_tasks)?;
    return Ok(());
}

pub fn read_tasks_from_file(file_path: &String) -> std::io::Result<Vec<Task>> {
    if !file_path_exists(file_path) {
        panic!("{file_path} not found");
    }

    let file = File::open(file_path)?;
    let tasks = from_reader(file)?;
    return Ok(tasks);
}