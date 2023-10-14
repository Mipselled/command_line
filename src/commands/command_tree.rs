use super::Command;
use std::fs;
use std::path::Path;

pub struct TreeCommand;

impl Command for TreeCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() > 1 {
            eprintln!("Usage: tree [directory_path]");
            return;
        }

        let start_dir = if args.len() == 1 {
            &args[0]
        } else {
            "./" // Default to the current directory if no path is specified
        };

        self.display_tree(start_dir, "", true);
    }

    fn command_name(&self) -> String {
        String::from("tree")
    }

    fn predefined_args(&self) -> Vec<String> {
        vec!["[directory_path]".to_string()]
    }
}

impl TreeCommand {
    fn display_tree(&self, path: &str, prefix: &str, is_last: bool) {
        let dir = Path::new(path);
        if dir.is_dir() {
            if let Some(dir_name) = dir.file_name().and_then(|os_str| os_str.to_str()) {
                let (branch, file) = if is_last { ("└─", " ") } else { ("├─", "│") };
                println!("{}{}{}", prefix, branch, dir_name);
        
                if let Ok(entries) = fs::read_dir(path) {
                    let entries: Vec<_> = entries.collect();
                    let entry_count = entries.len();
        
                    for (i, entry_result) in entries.into_iter().enumerate() {
                        if let Ok(entry) = entry_result {
                            let is_last = i == entry_count - 1;
                            let entry_path = entry.path();
                            let new_path = entry_path.to_str().unwrap();
                            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { file });
        
                            self.display_tree(&new_path, &new_prefix, is_last);
                        }
                    }
                }
            }
        }
    }     
}
