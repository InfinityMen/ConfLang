use crate::{
    lexer::structs::{
        Token,
        TokenType::{self, *},
    },
    parser::stmts::{
        RuleItem::{self, *},
        RuleMatch, Stmt,
    },
};

use crate::consts;

mod stmts;

#[derive(Clone)]
struct Rule {
    name: &'static str,
    rule_items: Vec<RuleItem>,
}

pub struct Parser {
    constr_rules: Vec<Rule>,
    expr_rules: Vec<Rule>,
    repl_rule: Rule,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            constr_rules: vec![
                Rule {
                    name: "FUNC_DEF",
                    rule_items: vec![
                        Token(TO),
                        Token(STUDY),
                        Token(THE),
                        Token(RITUAL),
                        Token(OF),
                        Ident,
                        Optional(vec![
                            Token(WITH),
                            List {
                                item: Box::new(Expr),
                                sep: COMMA,
                                last_sep: Some(AND),
                            },
                        ]),
                        Token(IS),
                        Token(A),
                        Token(PLEASURE),
                        Token(DOT),
                        Token(LBRACE),
                        CodeBlock,
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
                        Optional(vec![
                            Token(REGRADING),
                            List {
                                item: Box::new(Expr),
                                sep: COMMA,
                                last_sep: Some(AND),
                            },
                        ]),
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
                        Token(DOT),
                    ],
                },
            ],
            expr_rules: vec![
                Rule {
                    name: consts::INT,
                    rule_items: vec![Token(INT)],
                },
                Rule {
                    name: consts::FLOAT,
                    rule_items: vec![Token(FLOAT)],
                },
                Rule {
                    name: consts::STR,
                    rule_items: vec![Token(STR)],
                },
                Rule {
                    name: consts::BOOL,
                    rule_items: vec![Token(BOOL)],
                },
                Rule {
                    name: consts::VAR,
                    rule_items: vec![Token(VAR)],
                },
                Rule {
                    name: consts::ADD,
                    rule_items: vec![Expr, Token(HARMONIZED), Token(WITH), Expr],
                },
                Rule {
                    name: consts::SUB,
                    rule_items: vec![Expr, Token(DIMINISHED), Token(BY), Expr],
                },
                Rule {
                    name: consts::MUL,
                    rule_items: vec![Expr, Token(MULTIPLED), Token(BY), Expr],
                },
                Rule {
                    name: consts::DIV,
                    rule_items: vec![Expr, Token(SHARED), Token(AMONG), Expr],
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
                        Expr,
                    ],
                },
                Rule {
                    name: consts::DIV_WITH_REM,
                    rule_items: vec![Expr, Token(PARTITIONED), Token(BY), Expr],
                },
                Rule {
                    name: consts::REM_FROM_DIV,
                    rule_items: vec![
                        Expr,
                        Token(WITH),
                        Token(A),
                        Token(REMAINDER),
                        Token(OF),
                        Expr,
                    ],
                },
                Rule {
                    name: consts::FUNC_CALL_EXPR,
                    rule_items: vec![
                        Token(WISDOM),
                        Token(OF),
                        Ident,
                        Optional(vec![
                            Token(REGRADING),
                            List {
                                item: Box::new(Expr),
                                sep: COMMA,
                                last_sep: Some(AND),
                            },
                        ]),
                    ],
                },
            ],
            repl_rule: Rule {
                // для поддержки выражений как самостоятельных конструктов
                name: consts::EXPR,
                rule_items: vec![Expr],
            },
        }
    }

    pub fn parse_tokens(&self, tokens: Vec<Token>, repl_mode: bool) -> Vec<Stmt> {
        let mut p_rules = self.constr_rules.clone();
        let mut rem: Vec<Token>;
        let mut ast: Vec<Stmt> = Vec::new();
        let mut pos: usize = 0;

        if repl_mode {
            p_rules.push(self.repl_rule.clone());
        }

        while pos < tokens.len() {
            rem = tokens[pos..].to_vec();
            let mut best_rule: Option<(Rule, usize)> = None;

            println!("Pos:{}. Selecting rule for tokens", pos);

            for r in &p_rules {
                // выбор подходящего правила (с самым длинным совпадением)
                match self.check_rule(&rem, &r) {
                    RuleMatch::NoMatch => {
                        println!("rule: {}, NoMatch.", r.name);
                    }
                    RuleMatch::Match(ml) => {
                        println!("rule: {}, Match with len {}.", r.name, ml);
                        if let Some((ref m, bl)) = best_rule {
                            if ml > bl {
                                println!(
                                    "This rule have more match len than {} with len {}, selecting...",
                                    m.name, bl
                                );
                                best_rule = Some((r.clone(), ml))
                            }
                        } else {
                            println!("This is the firs checked rule, selecting...");
                            best_rule = Some((r.clone(), ml))
                        }
                    }
                }
            }

            if let Some(m) = best_rule {
                // match m.0.name {
                //     _ => todo!(),
                // }
                println!(
                    "Matched rule {}, length:{} tokens. Moving pos at {}, tokens.",
                    m.0.name, m.1, m.1
                );
                pos += m.1;
            } else {
                panic!("No one rule was matched in parser!")
            }
        }

        return ast;
    }

    fn get_accepted_tokens_list(&self, r: RuleItem) -> Vec<TokenType> {
        let mut ts = match r {
            Token(t) => vec![t],
            Ident => vec![VAR],
            Expr => vec![
                VAR,
                INT,
                FLOAT,
                STR,
                BOOL,
                HARMONIZED,
                WITH,
                DIMINISHED,
                BY,
                MULTIPLED,
                SHARED,
                AMONG,
                RAISED,
                TO,
                THE,
                POWER,
                OF,
                PARTITIONED,
                A,
                REMAINDER,
                WISDOM,
                REGRADING,
                LBRACE,
                RBRACE,
            ],
            List {
                item,
                sep,
                last_sep,
            } => {
                let mut ts: Vec<TokenType> = Vec::new();

                ts.extend(self.get_accepted_tokens_list(item.as_ref().clone()));
                ts.push(sep);

                if let Some(ls) = last_sep {
                    ts.push(ls);
                }

                ts
            }
            Optional(s) => s.iter().map(|&st| self.get_accepted_tokens_list(st)).collect(),
            Sequence(s) => {
                let mut ts: Vec<TokenType> = Vec::new();
                for r in s {
                    ts.extend(self.get_accepted_tokens_list(r));
                }
                ts
            }
            CodeBlock => unreachable!(), // поскольку парсер будет автоматически игнорировать содержимое скобок
        };

        ts.extend([NEWLINE, WHITESPACE]);
        ts

    }

    fn check_rule(&self, tokens: &Vec<Token>, rule: &Rule) -> RuleMatch {
        let mut m_len: usize = 0;

        for mr in &rule.rule_items {
            match self.check_item(tokens, m_len, mr) {
                Some(l) => m_len += l,
                None => return RuleMatch::NoMatch
            }
        }

        RuleMatch::Match(m_len)
    }

    fn check_item(&self, tokens: &Vec<Token>, pos: usize, item: &RuleItem) -> Option<usize> {
        match item {
            Token(tt) => {
                let tok = tokens.get(pos)?;

                if tok.token_type == *tt { Some(1) } else { None }
            }

            Ident | Expr => {
                let accepted = self.get_accepted_tokens_list(item.clone());

                let mut len = 0;

                while let Some(tok) = tokens.get(pos + len) {
                    if accepted.contains(&tok.token_type) {
                        len += 1;
                    } else {
                        break;
                    }
                }

                if len > 0 { Some(len) } else { None }
            }

            Optional(inner) => Some(self.check_item(tokens, pos, inner).unwrap_or(0)),

            Sequence(items) => {
                let mut consumed = 0;

                for item in items {
                    let len = self.check_item(tokens, pos + consumed, item)?;

                    consumed += len;
                }

                Some(consumed)
            }

            List {
                item,
                sep,
                last_sep,
            } => {
                let mut consumed = 0;

                let first = self.check_item(tokens, pos, item)?;

                consumed += first;

                loop {
                    let Some(tok) = tokens.get(pos + consumed) else {
                        break;
                    };

                    if tok.token_type == *sep
                        || last_sep.as_ref().is_some_and(|ls| *ls == tok.token_type)
                    {
                        consumed += 1;

                        let len = self.check_item(tokens, pos + consumed, item)?;

                        consumed += len;
                    } else {
                        break;
                    }
                }

                Some(consumed)
            }

            CodeBlock => {
                let mut depth = 0;
                let mut consumed = 0;

                while let Some(tok) = tokens.get(pos + consumed) {
                    match tok.token_type {
                        LBRACE => depth += 1,

                        RBRACE => {
                            if depth == 0 {
                                break;
                            }

                            depth -= 1;

                            if depth == 0 {
                                consumed += 1;
                                break;
                            }
                        }

                        _ => {}
                    }

                    consumed += 1;
                }

                Some(consumed)
            }
        }
    }
}
