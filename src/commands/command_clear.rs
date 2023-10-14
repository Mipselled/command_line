use super::Command; // Import the Command trait defined in the parent module

// Define a struct representing the "clear" command
pub struct ClearCommand;

impl Command for ClearCommand {
    // Implement the execute function for the "clear" command
    fn execute(&self, _args: Vec<String>) {
        // Clear the screen
        print! ("\x1B[2J\x1B[1;1H");

        // Print a header indicating the program version
        print!("Rust Command Line [Version {}]\n(c) Rust Community. All rights reserved.\n\n", env!("CARGO_PKG_VERSION"));
    }

    // Implement the predefined_args function to specify that there are no expected arguments
    fn predefined_args(&self) -> Vec<String> {
        vec![]
    }
}