use std::{
    fmt::{self},
    io,
};

#[derive(Debug)]
pub enum ShellError {
    Io(io::Error),
    Syntax(String),
    Builtin(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "I/O Error: {}", e),
            ShellError::Syntax(msg) => write!(f, "Syntax Error: {}", msg),
            ShellError::Builtin(msg) => write!(f, "Builtin Error: {}", msg),
        }
    }
}

impl From<io::Error> for ShellError {
    fn from(value: io::Error) -> Self {
        ShellError::Io(value)
    }
}

impl std::error::Error for ShellError {}
