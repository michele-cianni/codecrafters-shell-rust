use crate::builtins::cd::CdHandler;
use crate::builtins::echo::EchoHandler;
use crate::builtins::exit::ExitHandler;
use crate::builtins::pwd::PwdHandler;
use crate::builtins::typo::TypeHandler;
use crate::builtins::CommandHandler;
use crate::shell::command::{BuiltinCommand, CommandResult, CommandType};
use crate::shell::external::ExternalExecutor;
use CommandResult::Continue;
use CommandType::{Builtin, Empty, External};

pub struct Dispatcher {
    executor: ExternalExecutor,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            executor: ExternalExecutor::new(),
        }
    }

    pub fn dispatch(&self, command: CommandType) -> CommandResult {
        match command {
            Empty => Continue,
            Builtin(builtin, args) => self.dispatch_builtin(builtin, args),
            External(cmd, args) =>  self.executor.execute(&cmd, args),
        }
    }

    fn dispatch_builtin(&self, b: BuiltinCommand, args: Vec<String>) -> CommandResult {
        let handler: &dyn CommandHandler = match b {
            BuiltinCommand::Echo => &EchoHandler,
            BuiltinCommand::Type => &TypeHandler,
            BuiltinCommand::Cd => &CdHandler,
            BuiltinCommand::Pwd => &PwdHandler,
            BuiltinCommand::Exit => &ExitHandler,
        };
        
        handler.execute(args)
    }
    
}