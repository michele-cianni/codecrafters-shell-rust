use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::shell::command::CommandResult;

pub struct ExternalExecutor;

impl ExternalExecutor {
    pub fn new() -> Self {
        ExternalExecutor
    }

    pub fn execute(&self, cmd: &str, args: Vec<&str>) -> CommandResult {
        let Some(path) = find_in_path(cmd) else {
            println!("{cmd}: command not found");
            return CommandResult::Continue;
        };

        let file_name = path.file_name()
            .unwrap_or(path.as_os_str());

        match Command::new(file_name).args(args).spawn() {
            Ok(mut child) => {
                if let Err(err) = child.wait() {
                    println!("{cmd}: failed to wait for process to exit: {err}");
                }
            }
            Err(err) => println!("{cmd}: failed to execute: {err}"),
        }

        CommandResult::Continue
    }
}

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    env::var_os("PATH")
        .map(|p| env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|dir| dir.join(name))
        .find(|p| is_executable(p))
}

fn is_executable(path: &Path) -> bool {
    path.is_file()
        && path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}