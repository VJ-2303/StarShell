use std::{
    env,
    io::{self, Write},
};
mod error;
mod executor;
mod parser;
use executor::execute_command;

fn main() {
    if let Err(e) = ctrlc::set_handler(|| {
        println!();
    }) {
        eprintln!("Error setting Ctrl-C handler: {}", e);
    }
    loop {
        let current_dir = env::current_dir().unwrap_or_default();
        let mut prompt_path = current_dir.display().to_string();

        if let Ok(home) = env::var("HOME") {
            if prompt_path.starts_with(&home) {
                prompt_path = prompt_path.replacen(&home, "~", 1);
            }
        }
        print!("{} ❯ ", prompt_path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        if let Err(e) = execute_command(&args) {
            eprintln!("rsh: {}", e);
        }
    }
}
