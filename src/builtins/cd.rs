use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;
use std::env;
use std::path::PathBuf;

pub struct CdHandler;

const HOME: &'static str = "HOME";

impl CommandHandler for CdHandler {
    fn execute(&self, args: &[&str]) -> CommandResult {
        let Some(target) = args.first().copied() else {
            println!("cd: missing operand");
            return CommandResult::Continue;
        };

        let path = if target == "~" {
            match env::var_os(HOME) {
                Some(home) => PathBuf::from(home),
                None => {
                    println!("cd: could not determine home directory");
                    return CommandResult::Continue;
                }
            }
        } else if let Some(stripped) = target.strip_prefix("~/") {
            match env::var_os(HOME) {
                Some(home) => PathBuf::from(home).join(stripped),
                None => {
                    println!("cd: could not determine home directory");
                    return CommandResult::Continue;
                }
            }
        } else {
            PathBuf::from(target)
        };

        if !path.exists() || !path.is_dir() {
            println!("cd: {}: No such file or directory", path.display());
        } else if let Err(err) = env::set_current_dir(path) {
            println!("cd: error: {}", err);
        }
        CommandResult::Continue
    }
}