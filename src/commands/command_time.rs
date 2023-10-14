use crate::commands::get_command_map;

use super::Command;

use std::time::Instant;

pub struct TimeCommand;

impl Command for TimeCommand {
    fn execute(&self, args: Vec<String>) {
        
        let mut passed_args: Vec<String> = args;

        if passed_args.len() < 1 {
            println!("{:?}", passed_args);
            eprintln!("Usage: time {}", self.predefined_args().join(" "));
            return;
        }

        let command_to_time = passed_args[0].as_str();

        // Execute the custom command using your command map
        let command_map = get_command_map();
        let custom_command = command_map.get(command_to_time);

        match custom_command {
            Some(command) => {
                passed_args.remove(0); // Remove the command from our args list
                
                let start_time = Instant::now();
                command.execute(passed_args); // You can pass arguments here if needed

                let end_time = Instant::now();
                let elapsed_time = end_time.duration_since(start_time);
        
                println!("Execution Time: {:?}", elapsed_time);
            }
            None => {
                eprintln!("Unknown command: {}", command_to_time);
            }
        }
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["<command_to_time>".to_string()]
    }
}
