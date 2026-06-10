use crate::{
    lexer::structs::{TokenType::*},
    parser::stmts::{
        RuleItem::{self, *},
        Stmt,
    },
};

use crate::consts;

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
                        Everything,
                        Token(RBRACE),
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
                Rule {
                    name: "RETURN",
                    rule_items: vec![
                        Token(THE),
                        Token(BEGINNING),
                        Token(OF),
                        Token(WISDOM),
                        Token(IS),
                        Token(THE),
                        Token(DEFINITION),
                        Token(OF),
                        Token(TERMS),
                        Token(OF),
                        List {
                            item: Box::new(Expr),
                            sep: COMMA,
                            last_sep: Some(AND),
                        },
                        Token(DOT),
                    ],
                },
                Rule {
                    name: "PRINT",
                    rule_items: vec![
                        Token(HE),
                        Token(FIRST),
                        Token(PRESENTS),
                        Token(HIS),
                        Token(WORDS),
                        Token(AS),
                        List {
                            item: Box::new(Expr),
                            sep: COMMA,
                            last_sep: Some(AND),
                        },
                        Token(AND),
                        Token(THEN),
                        Token(ACCORDING),
                        Token(TO),
                        Token(THEM),
                        Token(HE),
                        Token(ACTS),
                        Token(DOT),
                    ],
                },
                Rule {
                    name: "ASSIGN",
                    rule_items: vec![
                        Token(HE),
                        Token(WHO),
                        Token(BY),
                        Token(REANIMATING),
                        List {
                            item: Box::new(Expr),
                            sep: COMMA,
                            last_sep: Some(AND),
                        },
                        Token(CAN),
                        Token(GAIN),
                        Token(KNOWLEGE),
                        Token(OF),
                        List {
                            item: Box::new(Ident),
                            sep: COMMA,
                            last_sep: Some(AND),
                        },
                    ],
                },
                Rule {
                    name: consts::INT,
                    rule_items: vec![
                        Token(INT)
                    ]
                },
                Rule {
                    name: consts::FLOAT,
                    rule_items: vec![
                        Token(FLOAT)
                    ]
                },
                Rule {
                    name: consts::STR,
                    rule_items: vec![
                        Token(STR)
                    ]
                },
                Rule {
                    name: consts::BOOL,
                    rule_items: vec![
                        Token(BOOL)
                    ]
                },
                Rule {
                    name: consts::VAR,
                    rule_items: vec![
                        Token(VAR)
                    ]
                },
                Rule {
                    name: consts::ADD,
                    rule_items: vec![
                        Expr,
                        Token(HARMONIZED),
                        Token(WITH),
                        Expr
                    ]
                },
                Rule {
                    name: consts::SUB,
                    rule_items: vec![
                        Expr,
                        Token(DIMINISHED),
                        Token(BY),
                        Expr
                    ]
                },
                Rule {
                    name: consts::MUL,
                    rule_items: vec![
                        Expr,
                        Token(MULTIPLED),
                        Token(BY),
                        Expr
                    ]
                },
                Rule {
                    name: consts::DIV,
                    rule_items: vec![
                        Expr,
                        Token(SHARED),
                        Token(AMONG),
                        Expr
                    ]
                },
                Rule {
                    name: consts::EXP,
                    rule_items: vec![
                        Expr,
                        Token(RAISED),
                        Token(TO),
                        Token(THE),
                        Token(POWER),
                        Token(OF),
                        Expr
                    ]
                },
                Rule {
                    name: consts::DIV_WITH_REM,
                    rule_items: vec![
                        Expr,
                        Token(PARTITIONED),
                        Token(BY),
                        Expr,
                    ]
                },
                Rule {
                    name: consts::REM_FROM_DIV,
                    rule_items: vec![
                        Expr,
                        Token(WITH),
                        Token(A),
                        Token(REMAINDER),
                        Token(OF),
                        Expr
                    ]
                },
                Rule {
                    name: consts::FUNC_CALL_EXPR,
                    rule_items: vec![
                        Token(WISDOM),
                        Token(OF),
                        Ident,
                        Token(REGRADING),
                        List { item: Box::new(Expr), sep: DOT, last_sep: Some(AND) }
                    ]
                }
            ],
        }
    }
}
