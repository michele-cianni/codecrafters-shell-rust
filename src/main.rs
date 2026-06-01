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
