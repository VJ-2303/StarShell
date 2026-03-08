use std::io::{self, Write};

fn main() {
    loop {
        print!("minishell > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        let args: Vec<&str> = command.split_whitespace().collect();

        println!("Parsed arguments: {:?}", args);
    }
}
