use std::io::{self, Write};

mod task;
mod file_handler;
mod command;

fn main() {
    command::display_help_message(command::HelpMessageOptions {
        error: false,
        end_program: false,
        entered_command: "".to_string(),
    });
    let mut all_tasks: Vec<task::Task> = Vec::new();

    loop {
        print!("\nEnter command: ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read command");

        let arguments: Vec<&str> = command.trim().split(" ").collect();

        if arguments.len() != 1 || command.trim() == "" {
            command::display_help_message(command::HelpMessageOptions {
                error: true,
                end_program: false,
                entered_command: "".to_string(),
            });
        }
        let arg: String = arguments[0].to_string();

        if !command::double_commands_are_valid(&arg) && !command::single_command_is_valid(&arg){
            command::display_help_message(command::HelpMessageOptions {
                error: true,
                end_program: false,
                entered_command: arg.clone(),
            });
            continue;
        }
        match arg.as_str() {
            "-help" => command::handle_help_command(),
            "-list" => command::handle_list_command(&all_tasks),
            "-exit" => command::handle_exit_command(),
            "-new" => command::handle_new_command(&mut all_tasks),
            "-mark-complete" => command::handle_mark_as_complete_command(&mut all_tasks),
            "-unmark-complete" => command::handle_unmark_as_complete_command(&mut all_tasks),
            "-save-to-file" => command::handle_save_to_file_command(&all_tasks),
            "-load-from-file" => command::handle_load_from_file_command(&mut all_tasks),
            _ => println!("Unknown command: {}", arg),
        }
        
    }
}