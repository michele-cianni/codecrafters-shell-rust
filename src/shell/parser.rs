use crate::shell::command::{BuiltinCommand, CommandType};

pub fn parse_command(line: &str) -> CommandType<'_> {
    if line.is_empty() {
        return CommandType::Empty;
    }

    let mut parts = line.split_whitespace();
    let Some(cmd) = parts.next() else {
        return CommandType::Empty;
    };

    let args = parts.collect();
    match parse_builtin(cmd) {
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