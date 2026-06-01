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

fn handle_command(command: &str) -> bool {
    if command.is_empty() {
        return true; // keep running
    }
    if command.starts_with("type") {
        let &other_command = &command["type".len()..].trim();
        if other_command.is_empty() {
            println!("type: missing operand");
        }
        if other_command == "echo" || other_command == "exit" || other_command == "type" {
            println!("{} is a shell builtin", other_command.trim());
        }
        else {
            println!("{}: not found", other_command.trim());
        }

        return true;
    }

    if command.starts_with("echo ") {
        let message = &command[5..];
        println!("{}", message);
        return true; // keep running
    }

    if command == "exit" {
        return false; // stop loop
    }

    println!("{command}: command not found");
    true
}

fn main() -> io::Result<()> {
    loop {
        print_prompt()?;
        let command = read_command_line()?;
        if !handle_command(&command) {
            break;
        }
    }
    Ok(())
}
