use std::{
    env,
    io::{self, Read, Write},
};
mod error;
mod executor;
mod parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use executor::execute_command;

fn read_line() -> String {
    enable_raw_mode().unwrap();
    let mut buffer = String::new();

    for byte in io::stdin().bytes() {
        let b = byte.unwrap();

        if b == 13 {
            print!("\r\n");
            break;
        } else {
            let c = b as char;
            buffer.push(c);
            print!("{}", c);
            io::stdout().flush().unwrap();
        }
    }
    disable_raw_mode().unwrap();

    buffer
}

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

        let input = read_line();

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        if let Err(e) = execute_command(&args) {
            eprintln!("rsh: {}", e);
        }
    }
}
