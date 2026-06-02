pub mod builtins;
pub mod shell;

use crate::shell::Shell;
use std::io::{self};

fn main() -> io::Result<()> {
    Shell::new().run()
}
