mod interpreter;
mod lexer;
mod errors;
mod consts;

use crate::{errors::ErrHandler, interpreter::cli};
use consts::Colors;
use colored::*;
use std::{env, fs, process};

use crate::lexer::lexer::Lexer;

fn read_file(file_path: String) -> String {
    let contents = match fs::read_to_string(&file_path) {
        Ok(text) => text,

        Err(error) => {
            eprintln!(
                "{}: Could not read file '{}'.",
                "Error".firm_color(),
                &file_path
            );
            eprintln!("System message: {}", error);
            process::exit(1); // Выходим из программы с кодом ошибки 1
        }
    };

    return contents;
}


fn main() {
    #[cfg(windows)]
    let _ = control::set_virtual_terminal(true);
    control::set_override(true);

    let args: Vec<String> = env::args().collect();

    if &args.len() == &1 {
        // Here will be the REPL mode
    } else if &args.len() == &2 {
        if &args[1] == &"--help" {
            println!("");
            println!("Welcome to ConfLang help center!");
            println!("Here are the commands you can use in the console:");
            println!(
                "\t\"{}\" - Runs a ConfLang module",
                "clg <path to the file>".firm_color()
            );
            println!(
                "\t\"{}\" - Runs ConfLang in REPL mode",
                "clg".firm_color()
            );
            println!(
                "\t\"{}\" - Opens the help page",
                "clg --help".firm_color()
            );
            println!(
                "\t\"{}\" - Runs without creating a .clg file",
                "clg --nocache <other args>".firm_color()
            );
            println!(
                "\t\"{}\" - Runs in debug mode",
                "clg --debug <other args>".firm_color()
            );
            println!(
                "\t\"{}\" - Shows author and unlicense information",
                "clg --author".firm_color()
            );
            println!(
                "{}",
                "\t\"clg --donate\" - Support me :)".grey()
            );
            println!("")
        } else if &args[1] == &"--debug" {
            // REPL mode (with debug)
        } else if &args[1] == &"--donate" {
            println!("");
            println!(
                "Thank you very much for wanting to support me. My project is in the public domain, so I would be very grateful for your support. The link is below."
            );
            println!("{}", "https://boosty.to".firm_color());
            println!("");
        } else if &args[1] == &"--author" {
            println!("");
            println!(
                "{}: Pavel (a. k. a. Paul, InfinityMen)",
                "Author".firm_color()
            );
            println!(
                "{}: The unlicense (Public domain, https://unlicense.org)",
                "License".firm_color()
            );
            println!("");
        } else if &args[1] == &"--paskhalko" {
            println!("У попа была собака, он её любил. Она съела кусок мяса, он её убил.");
        } else {
            let file_path = &args[1];

            println!("Reading file {}", &file_path);
            let code: &str = &read_file(file_path.to_string());
            println!("{}", &code);
            println!("--- РЕЗУЛЬТАТ РАБОТЫ ЛЕКСЕРА ---");
            let lexer = Lexer::new();
            let tokens = lexer.parse_code(&code, None, false);
            println!("{:?}", tokens);

            
        }
    }
}
