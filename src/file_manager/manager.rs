use std::path::PathBuf;

use crate::{
    consts::{self, Colors},
    errors::{ErrHandler, enums::ErrLevel},
    file_manager::{
        console::{self, ConsoleArg},
        file::FileId,
        sm::SourceManager,
        span::PosIndex,
    },
    lexer::lexer::Lexer,
    parser::Parser,
};

pub struct FileManager {
    sm: SourceManager,
    err_level: ErrLevel,
    repl_mode: bool,
    nocache: bool,
    err_h: ErrHandler,
    pos: PosIndex,
    lex: Lexer,
    parser: Parser,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            sm: SourceManager::new(),
            err_level: ErrLevel::Error,
            repl_mode: false,
            nocache: false,
            err_h: ErrHandler::new(),
            pos: PosIndex::new(),
            lex: Lexer::new(),
            parser: Parser::new(),
        }
    }

    pub fn push_file(&mut self, path: PathBuf) -> FileId {
        let file_id = self.sm.push_file(&path);

        let text = self.sm.get_file_src(file_id);

        // sending info about new file into other modules
        self.err_h.push_file(file_id);
        self.pos.push_file(text, file_id);

        file_id
    }

    fn exec_file(&mut self, id: FileId) {
        let tokens = match self.lex.lex_code(id, &self.sm, &self.pos) {
            Ok(t) => t,
            Err(_) => unreachable!("пока что"),
        };

        println!("{:?}, tokens:\n{:?}", id, tokens);

        println!("{:?}, started parsing.", id);
        let ast = self.parser.parse_tokens(tokens, self.repl_mode);

        println!("{:?}, AST:\n{:?}", id, ast)
    }

    // pub fn import_file(&self, id: FileId) {}

    pub fn execute_cli(&mut self, args: Vec<String>) {
        let mut was_clg = false;
        let mut was_path = false;
        let mut was_ind_arg = false;
        let mut was_nocache = false;
        let mut was_err_level = false;
        let mut file_path = PathBuf::new();

        let parser = console::ConsoleParser::new(args);
        let prep = parser.parse();

        for a in prep {
            match a {
                ConsoleArg::Clg => {
                    if !was_clg {
                        was_clg = true
                    } else {
                        panic!("Path to the interpreter was declared in console more than 1 time.")
                    }
                }
                ConsoleArg::File(f) => {
                    if !was_path {
                        file_path = f;
                        was_path = true
                    } else {
                        panic!(
                            "Path to the executable file was declared in console more than 1 time."
                        )
                    }
                }
                ConsoleArg::Help => {
                    if !was_ind_arg {
                        println!("");
                        println!("Welcome to ConfLang console help center!");
                        println!("Here are the commands you can use in the console:");
                        println!(
                            "\t\"{}\" - Runs a ConfLang module",
                            "clg <path to the file>".firm_color()
                        );
                        println!("\t\"{}\" - Runs ConfLang in REPL mode", "clg".firm_color());
                        println!("\t\"{}\" - Opens this page", "clg -help".firm_color());
                        println!(
                            "\t\"{}\" - Runs without creating a .clcache file",
                            "clg -nocache <other args>".firm_color()
                        );
                        println!(
                            "\t\"{}\" - Runs with a certain level of error display",
                            "clg [ -debug | -warn | -err ] <other args>".firm_color()
                        );
                        println!(
                            "\t\"{}\" - Shows unlicense information",
                            "clg -license".firm_color()
                        );
                        println!(
                            "\t\"{}\" - Shows credits",
                            "clg -credits".firm_color()
                        );
                        println!("{}", "\t\"clg -donate\" - Support us :)".grey());
                        println!("");

                        was_ind_arg = true
                    } else {
                        panic!("Inderpendent arguments were declared in console more than 1 time.")
                    }
                }
                ConsoleArg::Credits => {
                    if !was_ind_arg {
                        println!("Credits text was not writed yet");
                        was_ind_arg = true
                    } else {
                        panic!("Inderpendent arguments were declared in console more than 1 time.")
                    }
                }
                ConsoleArg::License => {
                    if !was_ind_arg {
                        println!("{}", consts::UNLICENSE_TEXT);
                        was_ind_arg = true
                    } else {
                        panic!("Inderpendent arguments were declared in console more than 1 time.")
                    }
                }
                ConsoleArg::Donate => {
                    if !was_ind_arg {
                        println!("");
                        println!(
                            "Thank you very much for wanting to support us. Our project is in the public domain, so we would be very grateful for your support. The link is below."
                        );
                        println!("{}", "https://boosty.to".firm_color());
                        println!("");
                        was_ind_arg = true
                    } else {
                        panic!("Inderpendent arguments were declared in console more than 1 time.")
                    }
                }
                ConsoleArg::About => {
                    if !was_ind_arg {
                        println!("");
                        println!(
                            "{}, ver. {}.", "ConfLang".firm_color(), consts::CUR_VER);
                        println!("For more information please type {}, {}, and {}.", "clg -help".firm_color(), "clg -license".firm_color(), "clg -authors".firm_color());
                        println!("");
                        was_ind_arg = true
                    } else {
                        panic!("Inderpendent arguments were declared in console more than 1 time.")
                    }
                }
                ConsoleArg::NoCache => {
                    if !was_nocache {
                        self.nocache = true;
                        was_nocache = true
                    } else {
                        panic!("Nocache argument was declared in console more than 1 time.")
                    }
                }
                ConsoleArg::Level(l) => {
                    if !was_err_level {
                        self.err_level = l;
                        was_err_level = true
                    } else {
                        panic!("Error level was declared in console more than 1 time.")
                    }
                }
            }
        }

        if was_path {
            let id = self.push_file(file_path);
            self.exec_file(id);
        } else if !was_ind_arg {
            self.repl_mode = true;
            println!("REPL mode was not realised yet :(")
        }
    }
}
