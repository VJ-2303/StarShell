use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use crate::error::ShellError;

pub struct ShellHistory {
    pub entries: Vec<String>,
    pub file_path: PathBuf,
}

impl ShellHistory {
    pub fn load() -> Result<Self, ShellError> {
        let mut path = PathBuf::new();
        if let Ok(home) = env::var("HOME") {
            path.push(home);
        }
        path.push(".rsh_history");

        let mut entries = Vec::new();

        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    entries.push(line);
                }
            }
        }

        Ok(ShellHistory {
            entries,
            file_path: path,
        })
    }
    pub fn add(&mut self, command: &str) -> Result<(), ShellError> {
        self.entries.push(command.to_string());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "{}", command)?;
        Ok(())
    }
}
