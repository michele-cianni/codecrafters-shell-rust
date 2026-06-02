use std::env;
use std::path::Path;
use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;

pub struct CdHandler;

impl CommandHandler for CdHandler {
    fn execute(&self, args: &[&str]) -> CommandResult {
        let Some(target) = args.first().copied() else {
            println!("cd: missing operand");
            return CommandResult::Continue;
        };

        let path = Path::new(target);

        if !path.exists() || !path.is_dir() {
            println!("cd: {}: No such file or directory", path.display());
        } else if let Err(err) = env::set_current_dir(path) {
            println!("cd: error: {}", err);
        }
        CommandResult::Continue
    }
}