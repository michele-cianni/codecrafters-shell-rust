use std::env;
use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;

pub struct PwdHandler;

impl CommandHandler for PwdHandler {
    fn execute(&self, _args: &[&str]) -> CommandResult {
        match env::current_dir() {
            Ok(dir) => println!("{}", dir.display()),
            Err(err) => println!("pwd: {err}"),
        }
        CommandResult::Continue
    }
}