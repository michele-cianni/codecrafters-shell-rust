use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;
use std::env;
use std::path::PathBuf;

pub struct CdHandler;

const HOME: &'static str = "HOME";

impl CdHandler {
    fn expand_target(target: &String) -> Option<PathBuf> {
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
    fn execute(&self, args: Vec<String>) -> CommandResult {
        let target = args.first().cloned().unwrap_or_else(|| "~".to_string());

        let Some(path) = Self::expand_target(&target) else {
            println!("cd: {}: No such file or directory", &target);
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