use std::io;
use std::io::{IsTerminal, Write};
use crate::shell::command::CommandResult;
use crate::shell::dispatch::{ Dispatcher };
use crate::shell::parser::parse_command;

pub struct Shell {
    dispatcher: Dispatcher,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            dispatcher: Dispatcher::new(),
        }
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if io::stdin().is_terminal() {
                self.print_prompt()?;
            }
            let line = self.read_command_line()?;
            let command = parse_command(&line);
            if self.dispatcher.dispatch(command) == CommandResult::Exit {
                break;
            }
        }
        Ok(())
    }

    fn print_prompt(&self) -> io::Result<()> {
        print!("$ ");
        io::stdout().flush()
    }

    fn read_command_line(&self) -> io::Result<String> {
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        Ok(command.trim().to_string())
    }
}

