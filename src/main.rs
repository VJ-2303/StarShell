use std::{
    env,
    io::{self, Write},
};
mod editor;
mod error;
mod executor;
mod parser;
use executor::execute_command;

use crate::editor::read_input;

fn main() {
    if let Err(e) = ctrlc::set_handler(|| {
        println!();
    }) {
        eprintln!("Error setting Ctrl-C handler: {}", e);
    }
    let mut history: Vec<String> = Vec::new();
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

        let input = match read_input(&history, &prompt) {
            Ok(inp) => inp,
            Err(e) => {
                eprintln!("\r\nrsh editor error: {}", e);
                continue;
            }
        };

        if !input.trim().is_empty() {
            history.push(input.clone());
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
