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

fn is_builtin(command: &str) -> bool {
    matches!(command, "exit" | "echo" | "type")
}

fn handle_type_command(args: &[&str]) {
    if args.is_empty() {
        println!("type: missing operand");
        return;
    }

    let target = args[0];

    if is_builtin(target) {
        println!("{target} is a shell builtin");
    } else {
        println!("{target}: not found");
    }
}

fn handle_echo_command(args: &[&str]) {
    let message = args.join(" ");
    println!("{message}");
}

fn handle_command(line: &str) -> bool {
    if line.is_empty() {
        return true; // keep running
    }

    let mut parts = line.split_whitespace();
    let Some(cmd) = parts.next() else { return true; };
    let args:Vec<&str> = parts.collect();

    match cmd {
        "type" => {
            handle_type_command(&args);
            return true;
        },
        "echo" => {
            handle_echo_command(&args);
            return true;
        }
        "exit" => {
            return false; // exit the shell
        }
        _ => {
            println!("{cmd}: command not found");
            true
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        print_prompt()?;
        let line = read_command_line()?;
        if !handle_command(&line) {
            break;
        }
    }
    Ok(())
}
