

mod consts;
mod enums;

use std;

use crate::{consts::Colors, errors::enums::{ErrType, Severity}};
use enums::ErrName;



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


#[derive(Debug, Clone)]
pub struct Error {

    pub err_type: ErrType,
    pub name: ErrName,
    pub severity: Severity,

    pub line: usize,
    pub column: usize,
    pub message: String

}


#[derive(Clone)]
pub struct ErrHandler {
    lines: Vec<String>,
    tabs: Vec<usize>,
    fatal_errors: Vec<Error>,
    errors: Vec<Error>,
    warnings: Vec<Error>
    
}

impl ErrHandler {
    pub fn new(lines_info: Vec<(&str, usize)>) -> Self {
        let lines = lines_info.iter().map(|l| l.0.to_string()).collect();
        let tabs: Vec<usize> = lines_info.iter().map(|l| l.1).collect();
        Self {
            lines: lines,
            tabs: tabs,
            fatal_errors: Vec::new(),
            errors: Vec::new(),
            warnings:Vec::new()
            
        }
        
    }

    fn get_err_name(&self, code: ErrName) -> &str {
        match code {
            ErrName::IncorrectSyntax => "SyntaxError",
            ErrName::UnusedCharacter => "UnusedCharacterError",
            ErrName::UnterminatedString => "UnterminatedStringError",
            ErrName::InvalidValue => "InvalidValueError",
            ErrName::NumberOverflow => "NumberOwerflowError",
            ErrName::MisssingBracket => "MissingBracketError",
            ErrName::InvalidExpression => "InvalidExpressionError",
            ErrName::UndefinedVariable => "UndefinedVariableError",
            ErrName::UndefinedFunction => "UndefinedFunction",
            ErrName::TypeMismatch => "TypeMismatchError",
            ErrName::DivisionByZero => "DivisionByZeroError",
            ErrName::InternalError => "InternalError",
            _ => "UnknownError",
        }
    }


    fn report_fatal(&self, err_type: ErrType, name: ErrName, line: usize, column: usize, msg: String) -> ! {
        std::process::exit(1)
    }

    fn report_warn(&self, err_type: ErrType, name: ErrName, line: usize, column: usize, msg: String) {
        todo!()
    }

    pub fn print_errs_by_sevrs(&self, severity: Severity, is_first: bool) {
        
        let mut errs_title = String::new();
        let mut errs = Vec::new();
        match severity {
            Severity::Warning => { 
                errs = self.warnings.clone();
                if !is_first {
                    errs_title = "The interpreter found these warnings. You should probably fix them.".to_string();
                } else {
                    errs_title = "Also, the interpreter found these warnings. You should probably fix them.".to_string();
                }
            },
            Severity::Fatal => { 
                errs = self.fatal_errors.clone();
                if !is_first {
                    errs_title = "The interpreter found this fatal error, which caused the program to be terminated urgently:".to_string();
                } else {
                    errs_title = "Also, the interpreter found this fatal error, which caused the program to be terminated urgently:".to_string();
                }
            }
        }

        eprintln!("{}", errs_title);

        for err in errs {
            let line = self.lines[err.line - 1].replace("\t", "    ");
            let line_def = format!("Main module, {}::{} : \"", err.line, err.column);
            let first_line = format!("{}{}\"", line_def, line);

            let mut cursor_line = String::new();

            for _ in 0..line_def.chars().count() {
                cursor_line.push(' ');
            };
            for _ in 0..err.column {
                cursor_line.push(' ');
            }
            cursor_line.push('^');


            eprintln!("{}", first_line);
            eprintln!("{}", cursor_line);
            eprintln!("{}: {}", self.get_err_name(err.name), err.message)


        }
    }


    pub fn syntax_err(&mut self, line_index: usize, column: usize) -> ! {
        self.report_fatal(
            ErrType::Lexer, 
            ErrName::IncorrectSyntax, 
            line_index, 
            column,
        format!("Incorrect syntax."));
    }


    pub fn unerminated_str_err(&mut self, line_index: usize, column: usize, val: String) -> ! {
        
        let mut val = val;
        self.report_fatal(
            ErrType::Lexer, 
            ErrName::UnterminatedString,
            line_index, 
            column, 
            format!("Missing closing quotation mark in String type \"{}\".", val.remove(0)));

    }

    pub fn invalid_value_err(&mut self, line_index: usize, column: usize, val: String) -> ! {
        self.report_fatal(
            ErrType::Lexer, 
            ErrName::InvalidValue, 
            line_index, 
            column, 
            format!("The numerical value {} does not match the writing format.", val));
    }

    pub fn number_overflow_err(&mut self, line_index: usize, column: usize, val: String) -> ! {
        self.report_fatal(
            ErrType::Lexer, 
            ErrName::NumberOverflow, 
            line_index, 
            column, 
            format!("The number {} is too large to be represented in 64-bit format.", val));
    }

    // pub fn invalid_expr_err(&mut self, line_index: usize, column: usize, expr: Vec<expr>) {


    //     match expr[0] {
    //         tok
    //     }



    //     self.report(
    //         ErrType::Lexer, 
    //         ErrName::NumberOverflow, 
    //         Severity::Error, 
    //         line_index, 
    //         column, 
    //         format!("The number {} is too large to be represented in 64-bit format.", val));
    // }

    pub fn internal_err(&mut self, line_index: usize, column: usize, message: String) {
        self.report_fatal(
            ErrType::Internal, 
            ErrName::InternalError, 
            line_index, 
            column, 
            format!("{}.", message));
    }


}