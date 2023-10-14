use super::Command;

pub struct TemplateCommand;

impl Command for TemplateCommand {
    fn execute(&self, args: Vec<String>) {
        if args.is_empty() {
            println!("Template executed without arguments.");
        } else {
            println!("Template executed with arguments: {:?}", args);

            let mut flags = Vec::new();
            let mut values = Vec::new();

            for arg in &args {
                if arg.starts_with("--") {
                    flags.push(arg.clone());
                } else {
                    values.push(arg.clone());
                }
            }

            self.process_flags(&flags);
            self.process_values(&values);

        }
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["[--flag]".to_string(), "[value]".to_string()]
    }

    // Override the default implementation for flag processing
    fn process_flags(&self, flags: &Vec<String>) {
        println!("Processing flags in template: {:?}", flags);
    }

    // Override the default implementation for value processing
    fn process_values(&self, values: &Vec<String>) {
        println!("Processing values in template: {:?}", values);
    }
}