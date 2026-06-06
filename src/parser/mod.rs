use std::{collections::VecDeque, ops::Index};

use crate::{
    lexer::structs::TokenType::*,
    parser::stmts::{
        RuleItem::{self, *},
        Stmt,
    },
};

mod stmts;

struct Rule {
    name: &'static str,
    rule_items: Vec<RuleItem>,
}

pub struct Parser {
    rules: Vec<Rule>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Rule {
                    name: "FUNC_DEF",
                    rule_items: vec![
                        Token(TO),
                        Token(STUDY),
                        Token(THE),
                        Token(RITUAL),
                        Token(OF),
                        Ident,
                        Optional(Box::new(Sequence(vec![
                            Token(WITH),
                            List {
                                item: Box::new(Expr),
                                sep: COMMA,
                                last_sep: Some(AND),
                            },
                        ]))),
                        Token(IS),
                        Token(A),
                        Token(PLEASURE),
                        Token(DOT),
                        Token(LBRACE),
                        Token(RBRACE)
                    ],
                },
                Rule {
                    name: "FUNC_CALL",
                    rule_items: vec![
                        Token(LET),
                        Token(US),
                        Token(CONSULT),
                        Token(THE),
                        Token(WISDOM),
                        Token(OF),
                        Ident,
                        Optional(Box::new(Sequence(vec![
                            Token(REGRADING),
                            List {
                                item: Box::new(Expr),
                                sep: COMMA,
                                last_sep: Some(AND),
                            },
                        ]))),
                        Token(DOT),
                    ],
                },
            ],
        }
    }
}
