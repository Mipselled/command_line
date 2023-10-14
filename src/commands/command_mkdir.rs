use super::Command; // Import the Command trait defined in the parent module
use std::fs; // Import the standard library's file system module

// Define a struct representing the "mkdir" command
pub struct MkDirCommand;

impl Command for MkDirCommand {
    // Implement the execute function for the "mkdir" command
    fn execute(&self, args: Vec<String>) {
        // Check if the correct number of arguments is provided
        if args.len() != 1 {
            eprintln!("Usage: mkdir {}", self.predefined_args().join(" "));
            return;
        }

        // Extract the directory name from the arguments
        let directory_name = &args[0];

        // Attempt to create a directory with the given name
        if fs::create_dir(directory_name).is_ok() {
            println!("Created directory: {}", directory_name);
        } else {
            eprintln!("Failed to create directory: {}", directory_name);
        }
    }

    // Implement the predefined_args function to specify the expected argument(s)
    fn predefined_args(&self) -> Vec<String> {
        vec!["<directory_name>".to_string()]
    }
}