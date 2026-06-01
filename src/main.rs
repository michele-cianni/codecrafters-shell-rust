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

fn main() {
    loop {
        print_prompt().unwrap();
        let command = read_command_line().unwrap();
        if command == "exit" {
            break;
        }
        print_command_not_found(&command);
    }
}
