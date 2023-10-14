use std::fs;
use super::Command;

/// A command to delete a file or directory.
pub struct DeleteCommand;

impl Command for DeleteCommand {
    /// Execute the delete command with the provided arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of string arguments, where the first argument is the path to delete.
    fn execute(&self, args: Vec<String>) {
        
        if args.is_empty() {
            eprintln!("Usage: tree {}", self.predefined_args().join(" "));
            return;
        }

        let path_to_delete = &args[0];

        if let Err(err) = self.delete_path(path_to_delete) {
            eprintln!("Error deleting {}: {}", path_to_delete, err);
        } else {
            println!("Deleted: {}", path_to_delete);
        }
    }

    /// Get predefined arguments for the delete command.
    fn predefined_args(&self) -> Vec<String> {
        vec!["<path_to_file_or_directory>".to_string()]
    }
}

impl DeleteCommand {
    /// Delete the specified path, whether it's a file or directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to delete.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `std::io::Error` if the deletion fails.
    fn delete_path(&self, path: &str) -> Result<(), std::io::Error> {
        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            fs::remove_file(path)?;
        } else if metadata.is_dir() {
            fs::remove_dir_all(path)?;
        }

        Ok(())
    }
}