use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Builtin {
    Exit,
    Echo,
    Type,
}
#[derive(Debug)]
enum Command<'a> {
    Builtin(Builtin, Vec<&'a str>),
    External(&'a str, Vec<&'a str>),
    Empty,
}

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()
}

fn read_command_line() -> io::Result<String> {
    let mut command = String::new();
    io::stdin().read_line(&mut command)?;
    Ok(command.trim().to_string())
}

fn parse_builtin(cmd: &str) -> Option<Builtin> {
    match cmd {
        "exit" => Some(Builtin::Exit),
        "echo" => Some(Builtin::Echo),
        "type" => Some(Builtin::Type),
        _ => None,
    }
}

fn parse_command(line: &str) -> Command<'_> {
    if line.is_empty() {
        return Command::Empty;
    }

    let mut parts = line.split_whitespace();
    let Some(cmd) = parts.next() else {
        return Command::Empty;
    };

    let args = parts.collect();
    match parse_builtin(cmd) {
        Some(builtin) => Command::Builtin(builtin, args),
        None => Command::External(cmd, args),
    }
}

fn paths_from_env() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|p| env::split_paths(&p).collect())
        .unwrap_or_default()
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.metadata() {
        Ok(metadata) => (metadata.permissions().mode() & 0o111) != 0,
        Err(_) => false,
    }
}

fn find_executable_in_path(command_name: &str) -> Option<PathBuf> {
    for dir in paths_from_env() {
        let candidate = dir.join(command_name);
        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn handle_type_command(args: &[&str]) {
    let Some(target) = args.first().copied() else {
        println!("type: missing operand");
        return;
    };

    if parse_builtin(target).is_some() {
        println!("{target} is a shell builtin");
        return;
    }

    match find_executable_in_path(target) {
        Some(path) => println!("{target} is {}", path.display()),
        None => println!("{target}: not found"),
    }
}

fn handle_echo_command(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn dispatch_command(command: Command<'_>) -> io::Result<bool> {
    match command {
        Command::Empty => Ok(true),
        Command::Builtin(Builtin::Exit, _) => Ok(false),
        Command::Builtin(Builtin::Echo, args) => {
            handle_echo_command(&args);
            Ok(true)
        }
        Command::Builtin(Builtin::Type, args) => {
            handle_type_command(&args);
            Ok(true)
        }
        Command::External(cmd, _args) => {
            println!("{cmd}: command not found");
            Ok(true)
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        print_prompt()?;
        let line = read_command_line()?;
        let command = parse_command(&line);
        if !dispatch_command(command)? {
            break;
        }
    }
    Ok(())
}
