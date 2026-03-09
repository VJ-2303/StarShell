use std::env;
use std::io;
use std::process::Command;

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
            let mut child = Command::new(args[0]).args(&args[1..]).spawn()?;
            child.wait()?;
        }
    }
    Ok(())
}
