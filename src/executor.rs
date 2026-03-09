use std::env;
use std::fs::File;
use std::io;
use std::process::{Command, Stdio};

pub fn execute_command(args: &[&str]) -> Result<(), io::Error> {
    match args[0] {
        "cd" => {
            if args.len() > 1 {
                env::set_current_dir(args[1])?;
            } else {
                eprintln!("cd requires an argument");
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            if let Some(pos) = args.iter().position(|&s| s == ">") {
                if pos + 1 >= args.len() {
                    eprintln!("syntax error: unexpected token at the end");
                    return Ok(());
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
