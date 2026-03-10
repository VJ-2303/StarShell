use crate::error::ShellError;
use crate::parser::{ParsedCommand, parse_command};
use std::env;
use std::fs::File;
use std::io;
use std::process::{Command, Stdio};

pub fn execute_command(args: &[&str]) -> Result<(), ShellError> {
    let parsed_cmd = parse_command(args)?;

    match parsed_cmd {
        ParsedCommand::Cd { path } => {
            env::set_current_dir(path)?;
        }
        ParsedCommand::Exit => {
            std::process::exit(0);
        }
        ParsedCommand::Normal { args } => {
            let mut child = Command::new(args[0]).args(&args[1..]).spawn()?;
            child.wait()?;
        }
        ParsedCommand::Redirect { args, filename } => {
            let file = File::create(filename)?;
            let mut child = Command::new(args[0])
                .args(&args[1..])
                .stdout(Stdio::from(file))
                .spawn()?;
            child.wait()?;
        }
        ParsedCommand::Pipe { left, right } => {
            let mut left_child = Command::new(left[0])
                .args(&left[1..])
                .stdout(Stdio::piped())
                .spawn()?;
            let left_stdout = left_child
                .stdout
                .take()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not capture stdout"))?;
            let mut right_child = Command::new(right[0])
                .args(&right[1..])
                .stdin(Stdio::from(left_stdout))
                .spawn()?;
            left_child.wait()?;
            right_child.wait()?;
        }
    }
    Ok(())
}
