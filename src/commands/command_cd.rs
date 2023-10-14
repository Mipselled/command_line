use std::env; // Import the standard library's environment module
use super::Command; // Import the Command trait defined in the parent module

// Define a struct representing the "cd" (change directory) command
pub struct CdCommand;

impl Command for CdCommand {
    // Implement the execute function for the "cd" command
    fn execute(&self, args: Vec<String>) {
        if args.len() != 1 {
            eprintln!("Usage: cd {}", self.predefined_args().join(" "));
            return;
        }

        // Extract the desired directory path from the arguments
        let destination = &args[0];

        // Get the current working directory
        let current_dir = env::current_dir().expect("Failed to retrieve the current working directory");

        // Create a mutable copy of the current directory
        let mut new_dir = current_dir.clone();

        if destination.starts_with("..") {
            // Navigate up multiple directory levels
            let levels = destination.chars().take_while(|&c| c == '.').count();
            for _ in 1..levels {
                new_dir.pop();
            }
        } else {
            if destination.starts_with("./") {
                // Append the path after "./" to the current directory
                new_dir.push(&destination[2..]);
            } else {
                // Change to the directory specified in the argument
                new_dir.push(destination);
            }
        }

        // Attempt to change the current working directory
        if let Err(err) = env::set_current_dir(new_dir) {
            eprintln!("Failed to change directory: {}", err);
        }
    }

    // Implement the predefined_args function to specify that there are no expected arguments
    fn predefined_args(&self) -> Vec<String> {
        vec!["<directory_path>".to_string()]
    }
}