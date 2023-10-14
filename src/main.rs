use regex::{self, Regex};
use std::io::{self, Write};
use std::env;

mod commands;
use commands::get_command_map;

fn main() {
    print!("Rust Command Line [Version {}] (c) Rust Community. All rights reserved.\n", env!("CARGO_PKG_VERSION"));
    let command_map = get_command_map();

    loop {
        let current_dir = env::current_dir().expect("Failed to retrieve the current working directory");
        print!("{}> ", current_dir.display());

        io::stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input 
            .trim()
            .to_ascii_lowercase();

        let regex = Regex::new(r#""(.*?)"|(\S+)"#).expect("Invalid regex");
        let args: Vec<String> = regex
            .captures_iter(&input)
            .filter_map(|cap| {
                cap.get(1)
                    .or_else(|| cap.get(2))
                    .map(|m| m.as_str().to_string())
            })
            .collect();

        if args.is_empty() {
            continue;
        }

        let command_name = &args[0];

        if command_name == "exit" {
            break;
        }

        let command_args = args[1..].to_vec();

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