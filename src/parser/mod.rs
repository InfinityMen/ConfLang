use core::panic;
use std::{todo, unreachable, vec};

use crate::{
    file_manager::{
        file::FileId,
        span::{self, PosIndex, Span},
    },
    lexer::structs::{Token, TokenType::*, TokenVal},
    parser::stmts::{
        RuleItem::{self, *},
        RuleMatch, Stmt, StmtContent, StmtType,
    },
};

use crate::consts;

mod stmts;

#[derive(Clone, Debug)]
struct Rule {
    name: &'static str,
    rule_items: Vec<RuleItem>,
}

pub struct Parser {
    constr_rules: Vec<Rule>,
    operators: Vec<Rule>,
    operands: Vec<Rule>,
    repl_rule: Rule,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            constr_rules: vec![
                Rule {
                    name: "FUNC_CALL",
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
                                last_sep: Some(WITH),
                            },
                        ]),
                        Token(IS),
                        Token(A),
                        Token(PLEASURE),
                        Token(DOT),
                        CodeBlock,
                    ],
                },
                Rule {
                    name: "FUNC_DEF",
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
                                last_sep: Some(WITH),
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
                            last_sep: Some(WITH),
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
                            last_sep: Some(WITH),
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
                            last_sep: Some(WITH),
                        },
                        Token(CAN),
                        Token(GAIN),
                        Token(KNOWLEGE),
                        Token(OF),
                        List {
                            item: Box::new(Ident),
                            sep: COMMA,
                            last_sep: Some(WITH),
                        },
                        Token(DOT),
                    ],
                },
            ],
            operators: vec![
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
            ],
            operands: vec![
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
                    name: consts::FUNC_CALL_EXPR,
                    rule_items: vec![
                        Token(WISDOM),
                        Token(OF),
                        Ident,
                        Token(REGRADING),
                        List {
                            item: Box::new(Expr),
                            sep: COMMA,
                            last_sep: Some(WITH),
                        },
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

    fn gen_stmt(&self, span: Span, content: StmtContent) -> Stmt {
        let stmt_type: StmtType = match content {
            StmtContent::Var { .. } => StmtType::Var,
            _ => todo!(),
        };

        return Stmt {
            s_type: stmt_type,
            content,
            span: span,
        };
    }

    pub fn parse_tokens(
        &self,
        id: FileId,
        posindx: &PosIndex,
        tokens: Vec<Token>,
        repl_mode: bool,
    ) -> Vec<Stmt> {
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
                match self.parse_rule(&rem, &r, false, id, posindx.clone()) {
                    RuleMatch::NoMatch(max_l, expected, found) => {
                        println!(
                            "rule: {:?}, NoMatch. Max matched len {}, expected {:?}, found {:?}",
                            r, max_l, expected, found
                        );
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
                    RuleMatch::CreateAST(_, _) => unreachable!(),
                }
            }

            if let Some(m) = best_rule {
                // match m.0.name {
                //     _ => todo!(),
                // }
                println!(
                    "Matched rule {}, length:{} tokens. Moving pos at {} tokens.",
                    m.0.name, m.1, m.1
                );
                pos += m.1;
            } else {
                panic!("No one rule was matched in parser!")
            }
        }

        return ast;
    }

    fn parse_expression(&self, tokens: &Vec<Token>, pos: usize) {
        let mut parse_scope = &tokens[pos..];
        let mut is_operator_expected = false;
        let mut cur_operator_pos: Option<[usize; 2]> = None;
        let mut best_operator: Option<String> = None;

        while !parse_scope.is_empty() {
            if is_operator_expected {
            } else {
                for r in self.operands.clone() {}
            }
        }
    }

    fn parse_rule(
        &self,
        tokens: &Vec<Token>,
        rule: &Rule,
        build_ast: bool,
        id: FileId,
        posindx: PosIndex,
    ) -> RuleMatch {
        let mut m_len: usize = 0;
        let mut check_scope = &tokens[m_len..];

        for ri in &rule.rule_items {
            while let Some(tok) = check_scope.get(m_len) {
                match tok.token_type {
                    WHITESPACE | NEWLINE => {
                        m_len += 1;
                    }
                    _ => break,
                }
            }

            m_len += match ri {
                Token(tt) => {
                    if check_scope.get(m_len).unwrap().token_type == *tt {
                        1
                    } else {
                        return RuleMatch::NoMatch(
                            m_len,
                            ri.clone(),
                            check_scope.get(m_len).unwrap().token_type.clone(),
                        );
                    }
                }
                Ident => {
                    match self.parse_ident(&check_scope.to_vec(), m_len, build_ast, id, &posindx) {
                        RuleMatch::Match(ml) => ml,
                        RuleMatch::NoMatch(ml, exp, found) => {
                            return RuleMatch::NoMatch(m_len + ml, exp, found);
                        }
                        _ => unreachable!(),
                    }
                }
                CodeBlock => match self.parse_codeblock(&check_scope.to_vec(), m_len, id, &posindx)
                {
                    RuleMatch::Match(ml) => ml,
                    RuleMatch::NoMatch(ml, exp, found) => {
                        return RuleMatch::NoMatch(m_len + ml, exp, found);
                    }
                    _ => unreachable!(),
                },
                _ => todo!(),
            };

            while let Some(tok) = check_scope.get(m_len) {
                match tok.token_type {
                    WHITESPACE | NEWLINE => {
                        m_len += 1;
                    }
                    _ => break,
                }
            }
        }

        RuleMatch::Match(m_len)
    }

    fn match_ruleitem(
        &self,
        ri: RuleItem,
        tokens: &[Token],
        pos: usize,
        id: FileId,
        posindx: &PosIndex,
        build_ast: bool,
    ) -> RuleMatch {
        let mut m_len: usize = 0;
        let mut check_scope = &tokens[m_len..];

        match ri {
                Token(ref tt) => {
                    if check_scope.get(m_len).unwrap().token_type == *tt {
                        1
                    } else {
                        return RuleMatch::NoMatch(
                            m_len,
                            ri.clone(),
                            check_scope.get(m_len).unwrap().token_type.clone(),
                        );
                    }
                }
                Ident => {
                    match self.parse_ident(&check_scope.to_vec(), m_len, build_ast, id, &posindx) {
                        RuleMatch::Match(ml) => ml,
                        RuleMatch::NoMatch(ml, exp, found) => {
                            return RuleMatch::NoMatch(m_len + ml, exp, found);
                        }
                        _ => unreachable!(),
                    }
                }
                CodeBlock => match self.parse_codeblock(&check_scope.to_vec(), m_len, id, &posindx)
                {
                    RuleMatch::Match(ml) => ml,
                    RuleMatch::NoMatch(ml, exp, found) => {
                        return RuleMatch::NoMatch(m_len + ml, exp, found);
                    }
                    _ => unreachable!(),
                },
                _ => todo!(),
            };

            unreachable!()

    }

    fn parse_codeblock(
        &self,
        tokens: &Vec<Token>,
        pos: usize,
        id: FileId,
        posindx: &PosIndex,
    ) -> RuleMatch {
        let parse_scope = &tokens[pos..];
        let mut len: usize = 0;
        let mut depth: u16 = 0;

        if parse_scope.first().unwrap().token_type != LBRACE {
            return RuleMatch::NoMatch(
                len,
                CodeBlock,
                parse_scope.first().unwrap().token_type.clone(),
            );
        }

        while let Some(t) = parse_scope.get(len) {
            match t.token_type {
                LBRACE => depth += 1,
                RBRACE => {
                    depth -= 1;
                    if depth == 0 {
                        len += 1;
                        break;
                    }
                }
                _ => {}
            }

            len += 1;
        }

        return RuleMatch::Match(len);
    }

    fn parse_ident(
        &self,
        tokens: &Vec<Token>,
        pos: usize,
        build_ast: bool,
        id: FileId,
        posindx: &PosIndex,
    ) -> RuleMatch {
        let parse_scope = &tokens[pos..];
        let mut m_len: usize = 0;
        // был ли педыдущий токен токеном переменной
        let mut was_var_token_prev = false;
        // были ли токены OF или FROM (нужно для проверки уместности использования токена FROM)
        let mut were_sense_tokens = false;
        // нужно для хранения пути файла при build_ast = true
        let mut ident_path: Vec<String> = vec![];

        while let Some(t) = parse_scope.get(m_len) {
            match t.token_type {
                VAR => {
                    if was_var_token_prev {
                        return RuleMatch::NoMatch(m_len, Ident, t.token_type.clone());
                    };
                    was_var_token_prev = true;
                    if build_ast {
                        match &t.val {
                            TokenVal::Var(v) => ident_path.push(v.to_string()),
                            _ => unreachable!(),
                        }
                    }
                }
                OF => {
                    if was_var_token_prev || were_sense_tokens {
                        return RuleMatch::NoMatch(m_len, Ident, t.token_type.clone());
                    }
                    was_var_token_prev = false;
                    were_sense_tokens = true
                }
                FROM => {
                    if was_var_token_prev {
                        return RuleMatch::NoMatch(m_len, Ident, t.token_type.clone());
                    }
                    was_var_token_prev = false;
                    were_sense_tokens = true
                }
                WHITESPACE | NEWLINE => {}
                _ => {
                    if !was_var_token_prev {
                        return RuleMatch::NoMatch(m_len, Ident, t.token_type.clone());
                    }
                    if build_ast {
                        return RuleMatch::CreateAST(
                            self.gen_stmt(
                                posindx.span_of_tokens(id, &parse_scope[m_len..]),
                                StmtContent::Var { name: ident_path },
                            ),
                            m_len,
                        );
                    }
                    return RuleMatch::Match(m_len);
                }
            }
            m_len += 1;
        }
        unreachable!()
    }

    // match item {
    //     Ident | Expr | List { .. } => {
    //         let accepted = self.get_accepted_tokens_list(item.clone());
    //         let mut micr_len: usize = 0;

    //         while let Some(tok) = check_scope.get(micr_len) {
    //             if accepted.contains(&tok.token_type) {
    //                 micr_len += 1;
    //             } else {
    //                 break;
    //             }
    //         }
    //         if micr_len != 0 {
    //             micr_len
    //         } else {
    //             return RuleMatch::NoMatch(
    //                 m_len + micr_len,
    //                 item.clone(),
    //                 check_scope.get(m_len).unwrap().token_type.clone(),
    //             );
    //         }
    //     }
    //     Optional(inner) => {
    //         let mut micr_len: usize = 0;

    //         for in_item in inner {
    //             match self.check_item(check_scope, micr_len, in_item) {
    //                 RuleMatch::Match(l) => micr_len += l,
    //                 RuleMatch::NoMatch(..) => {
    //                     micr_len = 0;
    //                     break;
    //                 }
    //             }
    //         }
    //         micr_len
    //     }
    // };

    // RuleMatch::Match(m_len)
    // }
}
