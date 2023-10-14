use super::Command;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

pub struct TreeCommand;

impl Command for TreeCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() > 1 {
            eprintln!("Usage: tree {}", self.predefined_args().join(" "));
            return;
        }

        let start_dir = if args.len() == 1 {
            // Convert the directory path to an OsStr
            OsStr::new(&args[0])
        } else {
            OsStr::new("./") // Default to the current directory if no path is specified
        };

        self.display_tree(start_dir, "", true);
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["[directory_path]".to_string()]
    }
}

impl TreeCommand {
    fn display_tree(&self, path: &OsStr, prefix: &str, is_last: bool) {
        let dir = Path::new(path);

        // Check if the path is a directory
        if dir.is_dir() {
            // Get the directory name as an OsStr
            if let Some(dir_name) = dir.file_name() {
                // Define characters to represent the tree structure
                let (branch, file) = if is_last { ("└─", " ") } else { ("├─", "│") };

                // Print the directory name with the appropriate prefix
                println!("{}{}{}", prefix, branch, dir_name.to_string_lossy());

                // Read the contents of the directory
                if let Ok(entries) = fs::read_dir(dir) {
                    let entries: Vec<_> = entries.collect();
                    let entry_count = entries.len();

                    for (i, entry_result) in entries.into_iter().enumerate() {
                        if let Ok(entry) = entry_result {
                            let is_last = i == entry_count - 1;
                            let entry_path = entry.path();

                            // Calculate the new prefix for the subdirectory
                            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { file });

                            // Recursively display the subdirectory
                            self.display_tree(entry_path.as_os_str(), &new_prefix, is_last);
                        }
                    }
                }
            }
        }
    }
}
