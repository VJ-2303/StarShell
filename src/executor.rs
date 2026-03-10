use crate::error::ShellError;
use std::env;
use std::fs::File;
use std::io;
use std::process::{Command, Stdio};

pub fn execute_command(args: &[&str]) -> Result<(), ShellError> {
    match args[0] {
        "cd" => {
            if args.len() > 1 {
                env::set_current_dir(args[1])?;
            } else {
                return Err(ShellError::Builtin("cd requires an argument.".to_string()));
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            if let Some(pos) = args.iter().position(|&s| s == "|") {
                if pos == 0 || pos + 1 >= args.len() {
                    return Err(ShellError::Syntax("unexpected token `|`".to_string()));
                }
                let left_args = &args[..pos];
                let right_args = &args[pos + 1..];

                let mut left_child = Command::new(left_args[0])
                    .args(&left_args[1..])
                    .stdout(Stdio::piped())
                    .spawn()?;

                let left_stdout = left_child.stdout.take().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        "Coult not capture stdout of left child",
                    )
                })?;

                let mut right_child = Command::new(right_args[0])
                    .args(&right_args[1..])
                    .stdin(Stdio::from(left_stdout))
                    .spawn()?;

                left_child.wait()?;
                right_child.wait()?;
            } else if let Some(pos) = args.iter().position(|&s| s == ">") {
                if pos + 1 >= args.len() {
                    return Err(ShellError::Syntax(
                        "missing filename for redirection.".to_string(),
                    ));
                }
                let filename = args[pos + 1];
                let command_args = &args[..pos];

                let file = File::create(filename)?;

                let mut child = Command::new(args[0])
                    .args(&command_args[1..])
                    .stdout(Stdio::from(file))
                    .spawn()?;
                child.wait()?;
            } else {
                let mut child = Command::new(args[0]).args(&args[1..]).spawn()?;
                child.wait()?;
            }
        }
    }
    Ok(())
}
