use std::collections::HashMap;
use crate::consts;
use crate::consts::Colors;


#[derive(Clone)]
pub struct ErrHandler {
    lines: Vec<String>,
}

impl ErrHandler {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines: lines,
            
        }
        
    }

    fn get_err_msg(&self, code: &str) -> &str {
        match code {
            consts::SYNTAX_ERROR => "Incorrect syntax.",
            consts::VALUE_OVERFLOW_ERROR => "The value is too large to be represented in 64-bit format.",
            consts::ZERO_DIVISION_ERROR => "Division by zero is impossible.",
            consts::NAME_EREROR => "Variable not found.",
            consts::TOKEN_NOT_FOUND_ERR => "Token not found.",
            _ => "Unknow error occured",
        }
    }

    fn get_note_msg(&self, code: &str) -> &str {
        match code {
            consts::TOKEN_NOT_FOUND_ERR => "If you see this, you are probably using a test build. If not, please report this error to me.",
            consts::VALUE_OVERFLOW_ERROR => "The maximum value of the Int type is 18,446,744,073,709,551,616",
            _ => "",
        }
    }

    fn errs_template(&self, code: &str, line_num: usize) {
            eprintln!("");
            if !self.get_note_msg(code).is_empty() {
                eprintln!("Line {}: \"{}\"\n\t{}: {}\n\t{}: {}", line_num, self.lines[line_num - 1], code.err(), self.get_err_msg(&code), "Note".firm_color(), self.get_note_msg(&code));
            } else {
                eprintln!("Line {}: \"{}\"\n\t{}: {}", line_num, self.lines[line_num - 1], code.err(), self.get_err_msg(&code));
            }
            eprintln!("");
            std::process::exit(1);
    }

    pub fn syntax_err(&self, line_num: usize) {
        self.errs_template(consts::SYNTAX_ERROR, line_num);
    }

    pub fn value_overflow_err(&self, line_num: usize) {
        self.errs_template(consts::SYNTAX_ERROR, line_num);
    }

    pub fn zero_division_err(&self, line_num: usize) {
        self.errs_template(consts::SYNTAX_ERROR, line_num);
    }

    pub fn name_err(&self, line_num: usize) {
        self.errs_template(consts::SYNTAX_ERROR, line_num);
    }

    pub fn token_not_found_err(&self, line_num: usize) {
        self.errs_template(consts::SYNTAX_ERROR, line_num);
    }

}



pub struct ConsoleErr {
    args: Vec<String>,
    line: String
}

impl ConsoleErr {

    fn quote_arg(arg: &str) -> String {
        if arg.contains(' ') || arg.contains('"') {
            let escaped = arg.replace('"', "\\\"");
            format!("\"{}\"", escaped)
        } else {
            arg.to_string()
        }
    }

   pub fn new(args: Vec<String> ) -> Self {

    let line = args.iter().map(|a| Self::quote_arg(a)).collect::<Vec<_>>().join(" ");

       Self { 
            args: args, 
            line: line, 
        }
    }

    fn errs_template(&self, code: &str) -> ! {
            eprintln!("");
            if !self.get_note_msg(code).is_empty() {
                eprintln!("Console input: \"{}\"\n\t{}: {}\n\t{}: {}", self.line, code.err(), self.get_err_msg(&code), "Note".firm_color(), self.get_note_msg(&code));
            } else {
                eprintln!("Console input: \"{}\"\n\t{}: {}", self.line, code.err(), self.get_err_msg(&code));
            }
            eprintln!("");
            std::process::exit(1);
    }

    fn get_err_msg(&self, code: &str) -> &str {
        match code {
            consts::MODULE_NOT_FOUND_ERROR => "This module not found.",
            consts::INCORRECT_ARGUMENT_ERROR => "Incorrect argument.",
            _ => "Unknow error occured",
        }
    }

    fn get_note_msg(&self, code: &str) -> &str {
        match code {
            consts::MODULE_NOT_FOUND_ERROR => "Check that the file path is spelled correctly. If the file path contains spaces, enclose it in quotation marks.",
            consts::INCORRECT_ARGUMENT_ERROR => "Check the spelling of the arguments. Available arguments can be found using the \"clg --help\" command.",
            _ => "",
        }
    }

    pub fn module_not_found_err(&self) -> ! {
        self.errs_template(consts::MODULE_NOT_FOUND_ERROR);
    }

    pub fn incorrect_argumet_err(&self) -> ! {
        self.errs_template(consts::INCORRECT_ARGUMENT_ERROR);
    }

}