use crate::shell::command::{BuiltinCommand, CommandType};

pub fn parse_command(line: &str) -> CommandType {
    let tokens = tokenize(line);
    if tokens.is_empty() {
        return CommandType::Empty;
    }

    let cmd = tokens[0].clone();
    let args = tokens[1..].to_vec();
    match parse_builtin(&cmd) {
        Some(builtin) => CommandType::Builtin(builtin, args),
        None => CommandType::External(cmd, args),
    }
}

pub fn parse_builtin(cmd: &str) -> Option<BuiltinCommand> {
    match cmd {
        "exit" => Some(BuiltinCommand::Exit),
        "echo" => Some(BuiltinCommand::Echo),
        "type" => Some(BuiltinCommand::Type),
        "pwd" => Some(BuiltinCommand::Pwd),
        "cd" => Some(BuiltinCommand::Cd),
        _ => None,
    }
}

pub fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            '\'' if !in_single_quote && !in_double_quote => in_single_quote = true,
            '\'' if in_single_quote => in_single_quote = false,
            '\"' if !in_double_quote && !in_single_quote => in_double_quote = true,
            '\"' if in_double_quote => in_double_quote = false,
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}