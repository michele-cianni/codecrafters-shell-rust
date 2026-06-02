use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;

pub struct ExitHandler;

impl CommandHandler for ExitHandler {
    fn execute(&self, _args: &[&str]) -> CommandResult {
        CommandResult::Exit
    }
}