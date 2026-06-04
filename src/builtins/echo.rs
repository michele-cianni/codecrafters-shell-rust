use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;

pub struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn execute(&self, args: Vec<String>) -> CommandResult {
        println!("{}", args.join(" "));
        CommandResult::Continue
    }
}