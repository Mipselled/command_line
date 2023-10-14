use super::Command; // Import the Command trait defined in the parent module
use std::fs; // Import the standard library's file system module

// Define a struct representing the "listdir" command
pub struct ListDirCommand;

impl Command for ListDirCommand {
    // Implement the execute function for the "listdir" command
    fn execute(&self, args: Vec<String>) {
        if args.is_empty() {
            // List the current directory if no arguments are provided
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        // Print the name of each entry in the current directory
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
            } else {
                eprintln!("Failed to list the current directory.");
            }
        } else {
            // Display an error message if any arguments are provided
            eprintln!("Usage: listdir");
        }
    }

    // Implement the predefined_args function to specify that there are no expected arguments
    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }
}