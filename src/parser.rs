use crate::error::ShellError;

#[derive(Debug)]
pub enum ParsedCommand<'a> {
    Cd {
        path: &'a str,
    },
    Exit,
    Normal {
        args: &'a [&'a str],
    },
    Redirect {
        args: &'a [&'a str],
        filename: &'a str,
    },
    Pipe {
        left: &'a [&'a str],
        right: &'a [&'a str],
    },
}

pub fn parse_command<'a>(args: &'a [&'a str]) -> Result<ParsedCommand<'a>, ShellError> {
    if let Some(pos) = args.iter().position(|&s| s == "|") {
        if pos == 0 || pos + 1 > args.len() {
            return Err(ShellError::Syntax("unexpected token `|`".to_string()));
        }
        return Ok(ParsedCommand::Pipe {
            left: &args[..pos],
            right: &args[pos + 1..],
        });
    }
    if let Some(pos) = args.iter().position(|&s| s == ">") {
        if pos == 0 || pos + 1 > args.len() {
            return Err(ShellError::Syntax(
                "missing filename for redirection".to_string(),
            ));
        }
        return Ok(ParsedCommand::Redirect {
            args: &args[..pos],
            filename: args[pos + 1],
        });
    }
    match args[0] {
        "cd" => {
            if args.len() > 1 {
                Ok(ParsedCommand::Cd { path: args[1] })
            } else {
                Err(ShellError::Builtin("cd requires an argument".to_string()))
            }
        }
        "exit" => Ok(ParsedCommand::Exit),
        _ => Ok(ParsedCommand::Normal { args }),
    }
}
