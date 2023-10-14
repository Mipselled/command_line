use super::Command;

pub struct ClearCommand;

impl Command for ClearCommand {
    fn execute(&self, _args: Vec<String>) {
        if cfg!(target_os = "windows") {
            // On Windows
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        } else {
            // On Unix-like systems
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }

        print!("Rust Command Line [Version {}] (c) Rust Community. All rights reserved.\n", env!("CARGO_PKG_VERSION"));
    }

    fn command_name(&self) -> String {
        return String::from("clear");
    }

    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }

}