use std::env;
use super::Command;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() != 1 {
            eprintln!("Usage: cd <directory_path>");
            return;
        }

        let destination = &args[0];

        let current_dir = env::current_dir().expect("Failed to retrieve the current working directory");
        let mut new_dir = current_dir.clone();

        if destination.starts_with("..") {
            // Navigate up multiple directory levels
            let levels = destination.chars().take_while(|&c| c == '.').count();
            for _ in 1..levels {
                new_dir.pop();
            }
        } else {
            if destination.starts_with("./") {
                new_dir.push(&destination[2..]);
            } else {
                new_dir.push(destination);
            }
        }

        if let Err(err) = env::set_current_dir(new_dir) {
            eprintln!("Failed to change directory: {}", err);
        }
    }


    fn command_name(&self) -> String {
        return String::from("cd");
    }

    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }

}