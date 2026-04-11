use crate::{errors::ErrHandler, lexer::lexer::Token, parser::stmts::Stmt};

mod stmts;

pub struct Parser {

    src_code: Vec<Token>,
    err_handler: ErrHandler

}

impl Parser {
    pub fn new(code: Vec<Token>, err_h: ErrHandler) -> Self {
        Self { 
            src_code: code,
            err_handler: err_h,
         }
    }

    pub fn parse_tokens(&self) -> Vec<Stmt> {
        return Vec::new();
    }
}