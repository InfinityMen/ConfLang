mod interpreter;
use std::{env, fs, process};
use colored::*;
use crate::interpreter::cli;

fn read_file(file_path: String) -> String {
    println!("In file {}", &file_path);

    let contents = match fs::read_to_string(&file_path) {
        Ok(text) => text,

        Err(error) => {
            eprintln!("{}: Could not read file '{}'.", "Error".bold().color("#00A2FF"), &file_path);
            eprintln!("System message: {}", error);
            process::exit(1); // Выходим из программы с кодом ошибки 1
        },
    };

    println!("With text:\n{contents}");
    return contents;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args.len() == &1 {
        // Here will be the REPL mode
    } else if &args.len() == &2 {
        if &args[1] == &"--help" {
            println!("");
            println!("Welcome to ConfLang help center!");
            println!("Here are the commands you can use in the console:");
            println!("\t\"{}\" - Runs a ConfLang module", "clg <path to the file>".bold().color("#00A2FF"));
            println!("\t\"{}\" - Runs ConfLang in REPL mode", "clg".bold().color("#00A2FF"));
            println!("\t\"{}\" - Opens the help page", "clg --help".bold().color("#00A2FF"));
            println!("\t\"{}\" - Runs without creating a .clc file", "clg --nocache <other args>".bold().color("#00A2FF"));
            println!("\t\"{}\" - Runs in debug mode", "clg --debug <other args>".bold().color("#00A2FF"));
            println!("\t\"{}\" - Shows author and unlicense information", "clg --author".bold().color("#00A2FF"));
            println!("{}", "\t\"clg --donate\" - Support me financially :)".color("#595959"));
            println!("")

        } else if &args[1] == &"--debug" {
            // REPL mode (with debug)
        } else if &args[1] == &"--donate" {
            println!("");
            println!("Thank you very much for wanting to support me. My project is in the public domain, so I would be very grateful for your support. The link is below.");
            println!("{}", "https://boosty.to".bold().color("#00A2FF"));
            println!("");
        } else if &args[1] == &"--author" {
            println!("");
            println!("{}: Pavel (a. k. a. Paul, InfinityMen)", "Author".bold().color("#00A2FF"));
            println!("{}: The unlicense (Public domain, https://unlicense.org)", "License".bold().color("#00A2FF"));
            println!("");
        } else if &args[1] == &"--paskhalko" {
            println!("У попа была собака, он её любил. Она съела кусок мяса, он её убил.")
        } else {
            let file_path = &args[1];

            println!("Reading file {}", &file_path);
            let code: &str = &read_file(file_path.to_string());
            cli::cli::interpret(&code);
        }
    }
}
