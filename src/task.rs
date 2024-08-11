use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub index: u8,
    pub description: String,    
    completed: bool,
}

pub trait Display {
    fn display_message(&self) -> String; 
}

impl Display for Task {
    fn display_message(&self) -> String {
        return format!("{}. {}, completed: {}", self.index, self.description, self.completed);
    }
}

impl Task {
    fn new(index: u8, description: String) -> Task {
        return Task {
            index: index,
            description: description,
            completed: false,
        }
    }
    pub fn mark_as_complete(&mut self) -> () {
        self.completed = true;
    }
    pub fn unmark_as_complete(&mut self) -> () {
        self.completed = false;
    }
}

pub fn list_all_tasks(all_tasks: &Vec<Task>) -> () {
    if all_tasks.len() == 0 {
        println!("No tasks found");
    } else {
        println!("To-do Tasks:\n");
        for task in all_tasks {
            println!("{}", task.display_message());
        }
    } 
}

pub fn create_new_task(all_tasks: &Vec<Task>, description: String) -> Task {
    let mut task_index: u8 = 0;

    if all_tasks.len() > 0 {
        let last_task: &Task = all_tasks.last().unwrap();
        task_index = last_task.index + 1;
    } 
    else {
        task_index = 1;
    }
    let new_task = Task::new(task_index, description);
    return new_task;
}