use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: Vec<String>) {
        if args.is_empty() {
            eprintln!("Usage: echo {}", self.predefined_args().join(" "));
            return;
        }

        let text: String = args.join(" ");
        println!("{}", text);
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["<text>".to_string()]
    }
}