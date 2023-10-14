use crate::commands::get_command_map;

use super::Command;

use std::time::Instant;

pub struct TimeCommand;

impl Command for TimeCommand {
    fn execute(&self, args: Vec<String>) {
        
        let mut passed_args: Vec<String> = args;

        if passed_args.len() < 1 {
            println!("{:?}", passed_args);
            eprintln!("Usage: time <command_to_time>");
            return;
        }

        let command_to_time = passed_args[0].as_str();

        let start_time = Instant::now();

        // Execute the custom command using your command map
        let binding = get_command_map();
        let custom_command = binding.get(command_to_time);
        match custom_command {
            Some(command) => {
                passed_args.remove(0);
                if passed_args.len() > 1 {
                    passed_args.remove(0);
                }
                command.execute(passed_args); // You can pass arguments here if needed
            }
            None => {
                eprintln!("Unknown command: {}", command_to_time);
            }
        }

        let end_time = Instant::now();
        let elapsed_time = end_time.duration_since(start_time);

        println!("Execution Time: {:?}", elapsed_time);
    }

    fn command_name(&self) -> String {
        return String::from("time");
    }

    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }
}
