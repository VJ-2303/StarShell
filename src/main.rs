use std::env; // NEW: Bring in environment manipulation tools
use std::io::{self, Write};
use std::process::Command;

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

        // NEW: Intercept built-in commands
        match args[0] {
            "cd" => {
                // cd requires a second argument (the destination)
                if args.len() > 1 {
                    // Safely attempt to change the directory
                    if let Err(e) = env::set_current_dir(args[1]) {
                        // eprintln! prints to the Standard Error pipe instead of Standard Out
                        eprintln!("cd error: {}", e);
                    }
                } else {
                    eprintln!("rsh: cd requires an argument");
                }
            }
            "exit" => {
                // Instantly terminate the shell with a success code (0)
                std::process::exit(0);
            }
            _ => {
                // DEFAULT: If it is not a builtin, spawn a child process
                let mut child = Command::new(args[0]).args(&args[1..]).spawn().unwrap(); // Still a temporary hack for spawning

                child.wait().unwrap(); // Still a temporary hack for waiting
            }
        }
    }
}
