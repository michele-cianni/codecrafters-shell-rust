#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

#[derive(Debug)]
pub enum CommandType<'a> {
    Builtin(BuiltinCommand, Vec<&'a str>),
    External(&'a str, Vec<&'a str>),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandResult {
    Continue,
    Exit,
}