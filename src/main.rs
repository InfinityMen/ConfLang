mod consts;
mod errors;
mod file_manager;
mod lexer;
mod parser;

use colored::*;
use std::env;

fn main() {
    #[cfg(windows)]
    let _ = control::set_virtual_terminal(true);
    control::set_override(true);

    let args: Vec<String> = env::args().collect();

    let mut manager = file_manager::manager::FileManager::new();

    manager.execute_cli(args);
}
