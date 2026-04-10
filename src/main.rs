mod consts;
mod errors;
mod interpreter;
mod lexer;
mod parser;

use crate::{errors::ConsoleErr, interpreter::cli};
use colored::*;
use consts::Colors;
use std::{env, fs};

enum DebugArgs {
    NoCache,
    Debug,
}

fn main() {
    #[cfg(windows)]
    let _ = control::set_virtual_terminal(true);
    control::set_override(true);

    let args: Vec<String> = env::args().collect();

    let console_err_handler = errors::ConsoleErr::new(args.clone());

    match args.len() {
        0 => {
            println!("Are u a {}?", "HACKER".err())
        }
        1 => {
            // REPL mode, where a u?
        }
        2 => {
            match args[1].as_str() {
                "--help" | "--h" => {
                    println!("");
                    println!("Welcome to ConfLang help center!");
                    println!("Here are the commands you can use in the console:");
                    println!(
                        "\t\"{}\" - Runs a ConfLang module",
                        "clg <path to the file>".firm_color()
                    );
                    println!("\t\"{}\" - Runs ConfLang in REPL mode", "clg".firm_color());
                    println!("\t\"{}\" - Opens the help page", "clg --help".firm_color());
                    println!(
                        "\t\"{}\" - Runs without creating a .clcache file",
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
                    println!("{}", "\t\"clg --donate\" - Support me :)".grey());
                    println!("")
                }

                "--debug" => {
                    // REPL with debug
                }

                "--donate" => {
                    println!("");
                    println!(
                        "Thank you very much for wanting to support me. My project is in the public domain, so I would be very grateful for your support. The link is below."
                    );
                    println!("{}", "https://boosty.to".firm_color());
                    println!("");
                }

                "--author" => {
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
                }

                "--paskhalko" => {
                    println!("У попа была собака, он её любил. Она съела кусок мяса, он её убил.");
                }

                _ => {
                    let file_path = args[1].clone();


                    let code = match fs::read_to_string(&file_path) {
                        Ok(text) => text,

                        Err(_) => {
                            console_err_handler.module_not_found_err();
                        }
                    };

                    interpreter::cli::interpret(&code, vec![]);
                }
            }
        }

        3 | 4 => {

            let mut cont_args: Vec<DebugArgs> = Vec::new();

            let mut rest_args: Vec<String> = args.iter().skip(2).cloned().collect();

            rest_args.retain(|arg| {
                match arg.as_str() {
                    "--debug" => {
                        cont_args.push(DebugArgs::Debug);
                        false
                    },

                    "--nocache" => {
                        cont_args.push(DebugArgs::NoCache);
                        false
                    },

                    _ => true
                }
            } );

            if !rest_args.is_empty() {
                console_err_handler.incorrect_argumet_err();
            }

        }
        _ => {}
    }
}
