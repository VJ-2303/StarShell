use std::io::{self, Read, Write};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::error::ShellError;

pub fn read_input(history: &[String], prompt: &str) -> Result<String, ShellError> {
    enable_raw_mode()?;

    let result = run_editor_loop(history, prompt);

    disable_raw_mode()?;

    result
}

fn run_editor_loop(history: &[String], prompt: &str) -> Result<String, ShellError> {
    let mut buffer = String::new();
    let mut history_index = history.len();
    let mut stdin_bytes = io::stdin().bytes();

    while let Some(byte_res) = stdin_bytes.next() {
        let b = byte_res?;

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
                io::stdout().flush()?;
            }
        } else if b == 27 {
            if let Some(bracket_res) = stdin_bytes.next() {
                if bracket_res? == 91 {
                    if let Some(dir_res) = stdin_bytes.next() {
                        let direction = dir_res?;

                        if direction == 65 {
                            if history_index > 0 {
                                history_index -= 1;
                                buffer = history[history_index].clone();
                                print!("\r\x1b[2K{}{}", prompt, buffer);
                                io::stdout().flush()?;
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
                                io::stdout().flush()?;
                            }
                        }
                    }
                }
            }
        } else {
            let c = b as char;
            print!("{}", c);
            buffer.push(c);
            io::stdout().flush()?;
        }
    }
    Ok(buffer)
}
