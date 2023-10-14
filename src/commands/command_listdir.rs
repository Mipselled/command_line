use super::Command;
use std::fs;

pub struct ListDirCommand;

impl Command for ListDirCommand {
    fn execute(&self, args: Vec<String>) {
        if args.is_empty() {
            // List the current directory if no arguments are provided
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
            } else {
                eprintln!("Failed to list the current directory.");
            }
        } else {
            eprintln!("Usage: listdir");
        }
    }

    fn command_name(&self) -> String {
        String::from("listdir")
    }

    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }
}
