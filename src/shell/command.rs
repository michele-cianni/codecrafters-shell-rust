#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

#[derive(Debug)]
pub enum CommandType {
    Builtin(BuiltinCommand, Vec<String>),
    External(String, Vec<String>),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandResult {
    Continue,
    Exit,
}