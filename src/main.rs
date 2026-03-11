use std::{
    env,
    io::{self, Write},
};
mod editor;
mod error;
mod executor;
mod history;
mod parser;
use executor::execute_command;

use crate::{editor::read_input, history::ShellHistory};

fn main() {
    if let Err(e) = ctrlc::set_handler(|| {
        println!();
    }) {
        eprintln!("Error setting Ctrl-C handler: {}", e);
    }

    let mut history = ShellHistory::load().unwrap_or_else(|_| {
        eprintln!("Warning: Could not load history file.");
        ShellHistory {
            entries: Vec::new(),
            file_path: std::path::PathBuf::new(),
        }
    });

    loop {
        let current_dir = env::current_dir().unwrap_or_default();
        let mut prompt_path = current_dir.display().to_string();

        if let Ok(home) = env::var("HOME") {
            if prompt_path.starts_with(&home) {
                prompt_path = prompt_path.replacen(&home, "~", 1);
            }
        }

        let prompt = format!("{} ❯ ", prompt_path);
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let input = match read_input(&history.entries, &prompt) {
            Ok(inp) => inp,
            Err(e) => {
                eprintln!("\r\nrsh editor error: {}", e);
                continue;
            }
        };

        if !input.trim().is_empty() {
            if let Err(e) = history.add(input.trim()) {
                eprintln!("Error saving history: {}", e)
            }
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        if let Err(e) = execute_command(&args) {
            eprintln!("rsh: {}", e);
        }
    }
}
