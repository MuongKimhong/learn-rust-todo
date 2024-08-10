use std::process;
use std::io::{self, Write};

use crate::task;
use crate::task::Task;
use crate::file_handler;

fn single_command_options() -> Vec<String> {
    return vec![
        String::from("-help"),
        String::from("-list"),
        String::from("-exit"),
    ];
}

// double command require additional argument
fn double_command_options() -> Vec<String> {
    return vec![
        String::from("-new"), // require description
        String::from("-mark-complete"), // require index
        String::from("-unmark-complete"), // require index
        String::from("-save-to-file"), // require /file/path
        String::from("-load-from-file"), // require /file/path
    ];
}

pub struct HelpMessageOptions {
    pub error: bool,
    pub end_program: bool,
    pub entered_command: String
}

pub fn display_help_message(options: HelpMessageOptions) -> () {
    if options.error {
        print!("\n--> Unknown command <{}>", options.entered_command);
    }

    let msg = "
    \n--> Please see all available command options below:\n\
    <-exit>   : exit the app,\n\
    <-help>   : display help message,\n\
    <-new>    : add new task, 'some description' is require,\n\
    <-list>   : list all to-do tasks,\n\
    <-mark-complete : mark task as complete, task id is require,\n\
    <-unmark-complete> : unmark task as complete, task id is require,\n\
    <-save-to-file> : write all tasks to file, /path/to/file is required,\n\
    <-load-from-file> : read tasks from file, /path/to/file is required,
    ";
    println!("{msg}");

    if options.end_program {
        process::exit(0);
    }
}

pub fn single_command_is_valid(arg: &String) -> bool {
    let single_arg: Vec<String> = single_command_options();
    return single_arg.contains(arg);
}

pub fn double_commands_are_valid(arg: &String) -> bool {
    let double_arg: Vec<String> = double_command_options();
    return double_arg.contains(arg); 
}

fn ask_description_input() -> String {
    print!("Enter description: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read description");
    return description.trim().to_string();
} 

fn ask_task_id_input() -> u8 {
    print!("Enter task id: ");
    io::stdout().flush().unwrap();

    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read id");

    let id: u8 = id.trim().parse().expect("Please enter a valid u8 number");
    return id;
}

fn ask_file_path_input() -> String {
    print!("Enter file path: ");
    io::stdout().flush().unwrap();

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read file path");
    return file_path.trim().to_string();
}

pub fn handle_help_command() -> () {
    display_help_message(HelpMessageOptions {
        error: false,
        end_program: false,
        entered_command: "".to_string(),
    });
}

pub fn handle_list_command(all_tasks: &Vec<Task>) -> () {
    task::list_all_tasks(all_tasks);
}

pub fn handle_new_command(all_tasks: &mut Vec<Task>) -> () {
    let description: String = ask_description_input();
    let new_task: Task = task::create_new_task(all_tasks, description);
    println!("{} is added to tasks", new_task.description);
    all_tasks.push(new_task); 
}

pub fn handle_mark_as_complete_command(all_tasks: &mut Vec<Task>) -> () {
    let task_id: u8 = ask_task_id_input();

    for task in all_tasks.iter_mut() {
        if task.index == task_id {
            task.mark_as_complete();
            println!("Task with {task_id} is marked as complete");
            return;
        }
    }
    println!("Task with id {} not found.", task_id);
}

pub fn handle_unmark_as_complete_command(all_tasks: &mut Vec<Task>) -> () {
    let task_id: u8 = ask_task_id_input();

    for task in all_tasks.iter_mut() { // iter_mut allow update element in Vec
        if task.index == task_id {
            task.unmark_as_complete();
            println!("Task with {task_id} is marked as uncomplete");
            return;
        }
    }
    println!("Task with id {} not found.", task_id);
}

pub fn handle_save_to_file_command(all_tasks: &Vec<Task>) -> () {
    let file_path: String = ask_file_path_input();
    let _ = file_handler::write_tasks_to_file(all_tasks, &file_path);
    println!("tasks are saved to {}", file_path);
}

pub fn handle_load_from_file_command(all_tasks: &mut Vec<Task>) -> () {
    let file_path: String = ask_file_path_input();
    let tasks = file_handler::read_tasks_from_file(&file_path).expect("Failed to load tasks from the file.");

    all_tasks.clear();
    all_tasks.extend(tasks);
    println!("tasks are loaded from {}", file_path);
}

pub fn handle_exit_command() -> () {
    print!("Exit app");
    process::exit(0);
}
