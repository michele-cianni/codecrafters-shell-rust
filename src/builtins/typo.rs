use crate::builtins::CommandHandler;
use crate::shell::command::CommandResult;
use crate::shell::external::find_in_path;
use crate::shell::parser::parse_builtin;

pub struct TypeHandler;

impl CommandHandler for TypeHandler {
    fn execute(&self, args: &[&str]) -> CommandResult {
        let Some(target) = args.first().copied() else {
            println!("type: missing operand");
            return CommandResult::Continue
        };

        if parse_builtin(target).is_some() {
            println!("{target} is a shell builtin");
            return CommandResult::Continue
        }

        match find_in_path(target) {
            Some(path) => println!("{target} is {}", path.display()),
            None => println!("{target}: not found"),
        }
        CommandResult::Continue
    }
}
