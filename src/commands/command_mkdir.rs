use super::Command;
use std::fs;

pub struct MkDirCommand;

impl Command for MkDirCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() != 1 {
            eprintln!("Usage: mkdir <directory_name>");
            return;
        }

        let directory_name = &args[0];

        if fs::create_dir(directory_name).is_ok() {
            println!("Created directory: {}", directory_name);
        } else {
            eprintln!("Failed to create directory: {}", directory_name);
        }
    }

    fn command_name(&self) -> String {
        return String::from("mkdir");
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["directory_name".to_string()]
    }
}
