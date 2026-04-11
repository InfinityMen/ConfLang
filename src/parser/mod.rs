use std::collections::VecDeque;

use crate::{errors::ErrHandler, lexer::lexer::Token, parser::stmts::Stmt};

mod stmts;

pub struct Parser {
    err_handler: ErrHandler,
    line_index: usize,
    depth: usize,
}

impl Parser {
    pub fn new(err_h: ErrHandler) -> Self {
        Self {
            err_handler: err_h,
            line_index: 0,
            depth: 0,
        }
    }

    pub fn parse_tokens(&mut self, code: Vec<Token>) -> Vec<Stmt> {
        let mut tokens = VecDeque::from(code.clone());
        let mut block_level: usize = 0;
        let mut expr_blocks: usize = 0;
        let cur_expr: Token;

        while !tokens.is_empty() {
            let token = tokens.pop_front();

            if let Some(t) = token {
                match t {

                    Token::FUNC_DEF(name) => {
                        let res = self.get_expr_brackets(Token::FUNC_DEF(name), tokens.clone());
                        tokens = res.0;
                    }

                    Token::NEWLINE => {
                        self.line_index += 1;
                    }

                    _ => {}
                }
            }
        }

        return Vec::new();
    }


    fn parse_func_def_args(&self, tokens: VecDeque<Token>) -> Vec<String> {
        let mut tokens = tokens;
        let mut args: Vec<String> = Vec::new();
        while !tokens.is_empty() {
            let token = tokens.pop_front();

            if let Some(t) = token {

                match t {
                    Token::VAR(arg) => {
                        args.push(arg);
                    },

                    Token::COMMA => {
                        continue;
                    }

                    _ => {

                    }
                }

            }
            
        }
        return args;
    }


    fn get_expr_brackets(

        &mut self,
        expr_token: Token,
        tokens: VecDeque<Token>,
    ) -> (VecDeque<Token>, Vec<VecDeque<Token>>) {


        let mut tokens = tokens;
        let need_root_brackets = self.get_root_brackets_num(expr_token);
        let mut found_root_brackets: usize = 0;
        let mut root_brackets: Vec<VecDeque<Token>> = Vec::new();

        while !tokens.is_empty() {
            if let Some(token) = tokens.pop_front() {
                match token {
                    Token::BLOCK_START => {
                        if self.depth == 0 {
                            root_brackets.push(VecDeque::new());
                        } else {
                            if let Some(last) = root_brackets.last_mut() {
                                last.push_back(token);
                            }
                        }

                        self.depth += 1;
                    }

                    Token::BLOCK_END => {
                        self.depth -= 1;

                        if self.depth == 0 {
                            found_root_brackets += 1;
                        } else {
                                if let Some(last) = root_brackets.last_mut() {
                                    last.push_back(token);
                                }
                            }

                            if need_root_brackets == found_root_brackets {
                                break;
                            
                        }
                    }

                    _ => {
                        if let Some(last) = root_brackets.last_mut() {
                            last.push_back(token);
                        }
                    }
                }
            }
        }

        return (tokens, root_brackets);
    }

    fn get_root_brackets_num(&self, token: Token) -> usize {
        match token {
            Token::FUNC_DEF(_) => 2,
            Token::FUNC_CALL(_) => 1,
            Token::FUNC_VOID(_) => 1,
            Token::RETURN => 1,
            Token::PRINT => 1,
            Token::ASSIGN => 2,
            _ => 0,
        }
    }
}
