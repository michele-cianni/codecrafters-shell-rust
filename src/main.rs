use std::env;
use std::io::IsTerminal;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
}
#[derive(Debug)]
enum CommandType<'a> {
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
        "pwd" => Some(Builtin::Pwd),
        _ => None,
    }
}

fn parse_command(line: &str) -> CommandType<'_> {
    if line.is_empty() {
        return CommandType::Empty;
    }

    let mut parts = line.split_whitespace();
    let Some(cmd) = parts.next() else {
        return CommandType::Empty;
    };

    let args = parts.collect();
    match parse_builtin(cmd) {
        Some(builtin) => CommandType::Builtin(builtin, args),
        None => CommandType::External(cmd, args),
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

fn handle_external_command(cmd: &str, args: Vec<&str>) -> io::Result<()> {
    let external_path = find_executable_in_path(cmd);
    if let Some(path) = external_path {
        Command::new(path.file_name().unwrap())
            .args(args)
            .spawn()?
            .wait()?;
    } else {
        println!("{cmd}: command not found");
    }
    Ok(())
}

fn dispatch_command(command: CommandType<'_>) -> io::Result<bool> {
    match command {
        CommandType::Empty => Ok(true),
        CommandType::Builtin(Builtin::Exit, _) => Ok(false),
        CommandType::Builtin(Builtin::Echo, args) => {
            handle_echo_command(&args);
            Ok(true)
        }
        CommandType::Builtin(Builtin::Type, args) => {
            handle_type_command(&args);
            Ok(true)
        }
        CommandType::Builtin(Builtin::Pwd, _) => {
            println!("{}", env::current_dir()?.display());
            Ok(true)
        }
        CommandType::External(cmd, args) => {
            handle_external_command(cmd, args)?;
            Ok(true)
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        if io::stdin().is_terminal() {
            print_prompt()?;
        }
        let line = read_command_line()?;
        let command = parse_command(&line);
        if !dispatch_command(command)? {
            break;
        }
    }
    Ok(())
}
