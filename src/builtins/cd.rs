use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;
use std::env;
use std::path::PathBuf;

pub struct CdHandler;

const HOME: &'static str = "HOME";

impl CdHandler {
    fn expand_target(target: &str) -> Option<PathBuf> {
        if target == "~" {
            env::var_os(HOME).map(PathBuf::from)
        } else if let Some(stripped) = target.strip_prefix("~/") {
            env::var_os(HOME).map(|home| PathBuf::from(home).join(stripped))
        } else {
            Some(PathBuf::from(target))
        }
    }
}

impl CommandHandler for CdHandler {
    fn execute(&self, args: &[&str]) -> CommandResult {
        let target = args.first().copied().unwrap_or("~");

        let Some(path) = Self::expand_target(target) else {
            println!("cd: {}: No such file or directory", target);
            return CommandResult::Continue;
        };

        if !path.exists() || !path.is_dir() {
            println!("cd: {}: No such file or directory", path.display());
        } else if let Err(err) = env::set_current_dir(path) {
            println!("cd: error: {}", err);
        }
        CommandResult::Continue
    }
}