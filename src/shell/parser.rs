use crate::shell::command::{BuiltinCommand, CommandType};
use std::mem::take;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuoteState {
    None,
    Single,
    Double,
    Backslash,
}

pub fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut state = QuoteState::None;

    let flush_current = |tokens: &mut Vec<String>, current: &mut String| {
        if !current.is_empty() {
            tokens.push(take(current));
        }
    };

    const SPACE: char = ' ';
    const TAB: char = '\t';
    const BACKSLASH: char = '\\';
    const SINGLE_QUOTE: char = '\'';
    const DOUBLE_QUOTE: char = '\"';

    for ch in line.chars() {
        match (state, ch) {
            // Outside quotes
            (QuoteState::None, SPACE | TAB) => flush_current(&mut tokens, &mut current),

            // Opening quotes
            (QuoteState::None, SINGLE_QUOTE) => state = QuoteState::Single,
            (QuoteState::None, DOUBLE_QUOTE) => state = QuoteState::Double,
            (QuoteState::None, BACKSLASH) => state = QuoteState::Backslash,

            // Closing quotes
            (QuoteState::Single, SINGLE_QUOTE) => state = QuoteState::None,
            (QuoteState::Double, DOUBLE_QUOTE) => state = QuoteState::None,
            (QuoteState::Backslash, ch) => {
                current.push(ch);
                state = QuoteState::None;
            }

            // Other chars
            _ => current.push(ch),
        }
    }

    flush_current(&mut tokens, &mut current);
    tokens
}
