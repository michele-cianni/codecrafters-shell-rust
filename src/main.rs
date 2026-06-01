#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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

fn get_command_in_path(command_name: &str, path: &PathBuf) -> PathBuf {
    let mut command_path = path.clone();
    command_path.push(command_name);
    command_path
}


fn handle_type_command(args: &[&str]) {
    if args.is_empty() {
        println!("type: missing operand");
        return;
    }

    let target = args[0];

    if is_builtin(target) {
        println!("{target} is a shell builtin");
        return;
    } else {
        let paths = paths_from_env();

        for p in paths {
            let buffer = &get_command_in_path(target, &p);
            let path = Path::new(buffer);
            if path.exists() && path.is_file() {
                if (path.metadata().unwrap().permissions().mode() & 0o111) != 0 {
                    println!("{} is {}", target, path.to_str().unwrap().to_string());
                    return;
                }
            }
        }
    }

    println!("{target}: not found");
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
    let Some(cmd) = parts.next() else {
        return true;
    };
    let args: Vec<&str> = parts.collect();

    match cmd {
        "type" => {
            handle_type_command(&args);
            true
        }
        "echo" => {
            handle_echo_command(&args);
            true
        }
        "exit" => {
            false // exit the shell
        }
        _ => {
            println!("{cmd}: command not found");
            true
        }
    }
}

fn paths_from_env() -> Vec<PathBuf> {
    match env::var_os("PATH") {

        Some(paths) => {
            env::split_paths(&paths).collect()
        }

        None => Vec::new(),
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
