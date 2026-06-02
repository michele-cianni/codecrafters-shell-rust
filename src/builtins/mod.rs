pub mod echo;
pub mod cd;
pub mod pwd;
pub mod typo;
pub mod exit;

use crate::shell::command::CommandResult;

pub trait CommandHandler {
    fn execute(&self, args: &[&str]) -> CommandResult;
}
