use std::env;
use std::io::{self, Write};
use std::process::Command;

fn execute_command(args: &[&str]) -> Result<(), io::Error> {
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

fn main() {
    loop {
        print!("rsh> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        if let Err(e) = execute_command(&args) {
            eprintln!("Error executing command: {}", e);
        }
    }
}
