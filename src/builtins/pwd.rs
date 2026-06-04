use std::env;
use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;

pub struct PwdHandler;

impl CommandHandler for PwdHandler {
    fn execute(&self, args: Vec<String>) -> CommandResult {
        if !args.is_empty() {
            println!("pwd: arguments not supported");
            return CommandResult::Continue;
        }

        match env::current_dir() {
            Ok(dir) => println!("{}", dir.display()),
            Err(err) => println!("pwd: {err}"),
        }
        CommandResult::Continue
    }
}