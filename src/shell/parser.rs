use crate::shell::command::{BuiltinCommand, CommandType};
use std::mem::take;

const SPACE: char = ' ';
const TAB: char = '\t';
const BACKSLASH: char = '\\';
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '\"';

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
    Backslash { in_quotes: bool },
}

impl QuoteState {
    fn next(self, ch: char, current: &mut String) -> Self {
        match (self, ch) {
            (QuoteState::None, SINGLE_QUOTE) => QuoteState::Single,
            (QuoteState::None, DOUBLE_QUOTE) => QuoteState::Double,
            (QuoteState::None, BACKSLASH) => QuoteState::Backslash { in_quotes: false },

            (QuoteState::Single, SINGLE_QUOTE) => QuoteState::None,
            (QuoteState::Double, DOUBLE_QUOTE) => QuoteState::None,
            (QuoteState::Double, BACKSLASH) => QuoteState::Backslash { in_quotes: true },

            (QuoteState::Backslash { in_quotes: false }, ch) => {
                current.push(ch);
                QuoteState::None
            }

            (QuoteState::Backslash { in_quotes: true }, ch) => {
                current.push(ch);
                QuoteState::Double
            }

            (_, ch) => {
                current.push(ch);
                self
            }
        }
    }

    fn is_token_separator(self, ch: char) -> bool {
        self == QuoteState::None && matches!(ch, SPACE | TAB)
    }
}

pub fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut state = QuoteState::None;

    for ch in line.chars() {
        if state.is_token_separator(ch) {
            flush_token(&mut tokens, &mut current);
        } else {
            state = state.next(ch, &mut current);
        }
    }

    flush_token(&mut tokens, &mut current);
    tokens
}

fn flush_token(tokens: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(take(current));
    }
}
