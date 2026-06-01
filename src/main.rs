#[allow(unused_imports)]
use std::io::{self, Write};

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()
}

fn read_command_line() -> io::Result<String> {
    let mut command = String::new();
    io::stdin().read_line(&mut command)?;
    Ok(command.trim().to_string())
}

fn print_command_not_found(command: &str) {
    println!("{command}: command not found");
}

fn main() -> io::Result<()> {
    print_prompt()?;
    let command = read_command_line()?;
    print_command_not_found(&command);
    Ok(())
}
