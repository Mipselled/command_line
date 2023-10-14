use regex::{self, Regex};
use std::io::{self, Write};
use std::env;

mod commands;
use commands::get_command_map;

//TODO: Proper flag and value handling
//TODO: Fix command_tree not working without a directory input
//TODO: Speed up performance

fn main() {
    // Print the program version
    print!("Rust Command Line [Version {}] (c) Rust Community. All rights reserved.\n", env!("CARGO_PKG_VERSION"));

    // Retrieve the command map, which maps command names to their respective functions
    let command_map = get_command_map();

    loop {
        // Get the current working directory
        let current_dir = env::current_dir().expect("Failed to retrieve the current working directory");
        print!("{}> ", current_dir.display());

        io::stdout().flush().expect("Unable to flush stdout");

        // Read user input from the command line
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Convert the input to lowercase and remove leading/trailing whitespace
        let input = input 
            .trim()
            .to_ascii_lowercase();

        // Define a regular expression for parsing input into arguments
        let regex = Regex::new(r#""(.*?)"|(\S+)"#).expect("Invalid regex");
        
        // Extract arguments from the input
        let args: Vec<String> = regex
            .captures_iter(&input)
            .filter_map(|cap| {
                cap.get(1)
                    .or_else(|| cap.get(2))
                    .map(|m| m.as_str().to_string())
            })
            .collect();

        // If no arguments are provided, continue to the next iteration of the loop
        if args.is_empty() {
            continue;
        }

        // Extract the command name (first argument)
        let command_name = &args[0];

        // Check if the user wants to exit the program
        if command_name == "exit" {
            break;
        }

        // Extract the command arguments (remaining arguments)
        let command_args = args[1..].to_vec();

        // Check if the entered command is valid and execute it
        match command_map.get(command_name.as_str()) {
            Some(command) => {
                command.execute(command_args);
            }
            None => {
                eprintln!("Unknown command: {}", command_name);
            }
        }
    }
}
