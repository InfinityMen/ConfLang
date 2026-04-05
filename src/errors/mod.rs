use std::collections::HashMap;
use crate::consts;
use crate::consts::Colors;


struct Error {
    name: &'static str,
    descr: &'static str,
}

pub struct ErrHandler {
    line_num: usize,
    line: String,
}

impl ErrHandler {
    pub fn new(line: String, line_num: usize) -> Self {
        Self {
            line_num: line_num, 
            line: line,
            
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
            consts::TOKEN_NOT_FOUND_ERR => "If you see this, you are probably using a test build. If not, please report this error to us.",
            consts::VALUE_OVERFLOW_ERROR => "The maximum value of the Int type is 18,446,744,073,709,551,616",
            _ => "",
        }
    }

    fn errs_template(&self, code: &str) {
            eprintln!("");
            if !self.get_note_msg(code).is_empty() {
                eprintln!("Line {}: \"{}\"\n\t{}: {}\n\t{}: {}", self.line_num, self.line, code.err(), self.get_err_msg(&code), "Note".firm_color(), self.get_note_msg(&code));
            } else {
                eprintln!("Line {}: \"{}\"\n\t{}: {}", self.line_num, self.line, code.err(), self.get_err_msg(&code));
            }
            eprintln!("");
            std::process::exit(1);
    }

    pub fn syntax_err(&self) {
        self.errs_template(consts::SYNTAX_ERROR);
    }

    pub fn value_overflow_err(&self) {
        self.errs_template(consts::VALUE_OVERFLOW_ERROR);
    }

    pub fn zero_division_err(&self) {
        self.errs_template(consts::ZERO_DIVISION_ERROR);
    }

    pub fn name_err(&self) {
        self.errs_template(consts::NAME_EREROR);
    }

    pub fn token_not_found_err(&self) {
        self.errs_template(consts::TOKEN_NOT_FOUND_ERR);
    }

}