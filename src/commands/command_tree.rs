use super::Command;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub struct TreeCommand;

impl Command for TreeCommand {
    fn execute(&self, args: Vec<String>) {
        if args.len() > 1 {
            eprintln!("Usage: tree {}", self.predefined_args().join(" "));
            return;
        }

        let current_dir = std::env::current_dir().expect("Failed to get current directory");

        let start_dir: &OsStr = if args.len() == 1 {
            OsStr::new(&args[0])
        } else {
            current_dir.as_os_str()
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

        if dir.is_dir() {
            if let Some(dir_name) = dir.file_name() {
                let (branch, file) = if is_last { ("└─", " ") } else { ("├─", "│") };
                println!("{}{}{}", prefix, branch, dir_name.to_string_lossy());

                if let Ok(entries) = fs::read_dir(dir) {
                    let entries: Vec<_> = entries.collect();
                    let entry_count = entries.len();

                    for (i, entry_result) in entries.into_iter().enumerate() {
                        if let Ok(entry) = entry_result {
                            let is_last = i == entry_count - 1;
                            let entry_path = entry.path();
                            let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { file });
                            self.display_tree(entry_path.as_os_str(), &new_prefix, is_last);
                        }
                    }
                }
            }
        }
    }
}