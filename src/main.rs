use std::{
    env,
    io::{self, Read, Write},
};
mod error;
mod executor;
mod parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use executor::execute_command;

fn read_input(history: &[String], prompt: &str) -> String {
    enable_raw_mode().unwrap();

    let mut buffer = String::new();
    let mut stdin_bytes = io::stdin().bytes();

    let mut history_index = history.len();

    while let Some(Ok(b)) = stdin_bytes.next() {
        if b == 13 {
            print!("\r\n");
            break;
        } else if b == 3 {
            print!("^C\r\n");
            buffer.clear();
            break;
        } else if b == 127 {
            if !buffer.is_empty() {
                buffer.pop();
                print!("\x08 \x08");
                io::stdout().flush().unwrap();
            }
        } else if b == 27 {
            if let Some(Ok(bracket)) = stdin_bytes.next() {
                if bracket == 91 {
                    if let Some(Ok(direction)) = stdin_bytes.next() {
                        if direction == 65 {
                            if history_index > 0 {
                                history_index -= 1;
                                buffer = history[history_index].clone();
                                print!("\r\x1b[2K{}{}", prompt, buffer);
                                io::stdout().flush().unwrap();
                            }
                        } else if direction == 66 {
                            if history_index < history.len() {
                                history_index += 1;
                                if history_index == history.len() {
                                    buffer.clear();
                                } else {
                                    buffer = history[history_index].clone();
                                }
                                print!("\r\x1b[2K{}{}", prompt, buffer);
                                io::stdout().flush().unwrap();
                            }
                        }
                    }
                }
            }
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

        let input = read_input(&history, &prompt);

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
