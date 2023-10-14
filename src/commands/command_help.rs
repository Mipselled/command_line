use crate::commands::get_command_map;

use super::Command;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, _args: Vec<String>) {
        println!("Available commands:");
        let command_map = get_command_map();
        for (key, value) in command_map {
            println!("{} {}", key, value.predefined_args().join(" "));
        }
    
    }

    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }

}