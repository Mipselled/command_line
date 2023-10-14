use std::collections::HashMap;

pub mod command_template;
pub mod command_clear;
pub mod command_time;
pub mod command_cd;
pub mod command_mkdir;
pub mod command_listdir;
pub mod command_tree;

pub fn get_command_map() -> HashMap<&'static str, Box<dyn Command>> {
    let mut command_map = HashMap::new();
    command_map.insert("template", Box::new(command_template::TemplateCommand) as Box<dyn Command>);
    command_map.insert("clear", Box::new(command_clear::ClearCommand) as Box<dyn Command>);
    command_map.insert("time", Box::new(command_time::TimeCommand) as Box<dyn Command>);
    command_map.insert("cd", Box::new(command_cd::CdCommand) as Box<dyn Command>);
    command_map.insert("mkdir", Box::new(command_mkdir::MkDirCommand) as Box<dyn Command>);
    command_map.insert("listdir", Box::new(command_listdir::ListDirCommand) as Box<dyn Command>);
    command_map.insert("tree", Box::new(command_tree::TreeCommand) as Box<dyn Command>);
    return command_map;
}


pub trait Command {
    fn execute(&self, args: Vec<String>);

    fn predefined_args(&self) -> Vec<String>;

    // Process flags
    fn process_flags(&self, _flags: &Vec<String>) {
        // Default implementation does nothing; individual commands can override this
    }

    // Process values
    fn process_values(&self, _values: &Vec<String>) {
        // Default implementation does nothing; individual commands can override this
    }

    fn command_name(&self) -> String;

}
