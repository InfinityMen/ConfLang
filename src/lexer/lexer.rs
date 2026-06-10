use super::structs::{Token, TokenType};
use crate::{
    consts::{self},
    errors::{ErrHandler, Error},
    file_manager::{file::FileId, sm::SourceManager, span::{PosIndex, Span}}, lexer::structs::{TokenType::NEWLINE, TokenVal},
};
use core::f64;
use regex::{Match, Regex};
use std::{num::IntErrorKind, vec};

pub struct Rule {
    name: &'static str,
    re: Regex,
}

pub struct Lexer {
    rules: Vec<Rule>,
    fictive_re: Regex,
    // err: ErrHandler,
    cur_span: Option<Span>,
    is_string: bool,
    is_opened_id: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            rules: vec![
                // whitespace
                Rule {
                    name: consts::NEWLINE,
                    re: Regex::new(r"^(?:\r\n|\n|\r)").unwrap(),
                },
                Rule {
                    name: consts::WHITESPACE,
                    re: Regex::new(r"^[ \t]+").unwrap(),
                },
                // func_def
                Rule {
                    name: consts::TO,
                    re: Regex::new(r"(?i)^To\b").unwrap(),
                },
                Rule {
                    name: consts::STUDY,
                    re: Regex::new(r"(?i)^study\b").unwrap(),
                },
                Rule {
                    name: consts::THE,
                    re: Regex::new(r"(?i)^the\b").unwrap(),
                },
                Rule {
                    name: consts::RITUAL,
                    re: Regex::new(r"(?i)^ritual\b").unwrap(),
                },
                Rule {
                    name: consts::OF,
                    re: Regex::new(r"(?i)^of\b").unwrap(),
                },
                Rule {
                    name: consts::WITH,
                    re: Regex::new(r"(?i)^with\b").unwrap(),
                },
                Rule {
                    name: consts::IS,
                    re: Regex::new(r"(?i)^is\b").unwrap(),
                },
                Rule {
                    name: consts::A,
                    re: Regex::new(r"(?i)^a\b").unwrap(),
                },
                Rule {
                    name: consts::PLEASURE,
                    re: Regex::new(r"(?i)^pleasure\b").unwrap(),
                },
                // func_call
                Rule {
                    name: consts::LET,
                    re: Regex::new(r"(?i)^let\b").unwrap(),
                },
                Rule {
                    name: consts::US,
                    re: Regex::new(r"(?i)^us\b").unwrap(),
                },
                Rule {
                    name: consts::CONSULT,
                    re: Regex::new(r"(?i)^consult\b").unwrap(),
                },
                Rule {
                    name: consts::WISDOM,
                    re: Regex::new(r"(?i)^wisdom\b").unwrap(),
                },
                Rule {
                    name: consts::REGRADING,
                    re: Regex::new(r"(?i)^regarding\b").unwrap(),
                },
                // return
                Rule {
                    name: consts::BEGINNING,
                    re: Regex::new(r"(?i)^beginning\b").unwrap(),
                },
                Rule {
                    name: consts::DEFINITION,
                    re: Regex::new(r"(?i)^definition\b").unwrap(),
                },
                Rule {
                    name: consts::TERMS,
                    re: Regex::new(r"(?i)^terms\b").unwrap(),
                },
                // print
                Rule {
                    name: consts::HE,
                    re: Regex::new(r"(?i)^he\b").unwrap(),
                },
                Rule {
                    name: consts::FIRST,
                    re: Regex::new(r"(?i)^first\b").unwrap(),
                },
                Rule {
                    name: consts::PRESENTS,
                    re: Regex::new(r"(?i)^presents\b").unwrap(),
                },
                Rule {
                    name: consts::HIS,
                    re: Regex::new(r"(?i)^his\b").unwrap(),
                },
                Rule {
                    name: consts::WORDS,
                    re: Regex::new(r"(?i)^words\b").unwrap(),
                },
                Rule {
                    name: consts::AS,
                    re: Regex::new(r"(?i)^as\b").unwrap(),
                },
                Rule {
                    name: consts::THEN,
                    re: Regex::new(r"(?i)^then\b").unwrap(),
                },
                Rule {
                    name: consts::ACCORDING,
                    re: Regex::new(r"(?i)^according\b").unwrap(),
                },
                Rule {
                    name: consts::THEM,
                    re: Regex::new(r"(?i)^them\b").unwrap(),
                },
                Rule {
                    name: consts::ACTS,
                    re: Regex::new(r"(?i)^acts\b").unwrap(),
                },
                // --- assign ---
                Rule {
                    name: consts::WHO,
                    re: Regex::new(r"(?i)^who\b").unwrap(),
                },
                Rule {
                    name: consts::BY,
                    re: Regex::new(r"(?i)^by\b").unwrap(),
                },
                Rule {
                    name: consts::REANIMATING,
                    re: Regex::new(r"(?i)^reanimating\b").unwrap(),
                },
                Rule {
                    name: consts::CAN,
                    re: Regex::new(r"(?i)^can\b").unwrap(),
                },
                Rule {
                    name: consts::GAIN,
                    re: Regex::new(r"(?i)^gain\b").unwrap(),
                },
                Rule {
                    name: consts::KNOWLEGE,
                    re: Regex::new(r"(?i)^knowledge\b").unwrap(),
                },
                // --- операторы (по словам) ---
                Rule {
                    name: consts::HARMONIZED,
                    re: Regex::new(r"(?i)^harmonized\b").unwrap(),
                },
                Rule {
                    name: consts::DIMINISHED,
                    re: Regex::new(r"(?i)^diminished\b").unwrap(),
                },
                Rule {
                    name: consts::MULTIPLED,
                    re: Regex::new(r"(?i)^multiplied\b").unwrap(),
                },
                Rule {
                    name: consts::SHARED,
                    re: Regex::new(r"(?i)^shared\b").unwrap(),
                },
                Rule {
                    name: consts::AMONG,
                    re: Regex::new(r"(?i)^among\b").unwrap(),
                },
                Rule {
                    name: consts::RAISED,
                    re: Regex::new(r"(?i)^raised\b").unwrap(),
                },
                Rule {
                    name: consts::POWER,
                    re: Regex::new(r"(?i)^power\b").unwrap(),
                },
                Rule {
                    name: consts::PARTITIONED,
                    re: Regex::new(r"(?i)^partitioned\b").unwrap(),
                },
                Rule {
                    name: consts::REMAINDER,
                    re: Regex::new(r"(?i)^remainder\b").unwrap(),
                },
                // --- служебные ---
                Rule {
                    name: consts::AND,
                    re: Regex::new(r"(?i)^and\b").unwrap(),
                },
                // --- bool ---
                Rule {
                    name: consts::YANG,
                    re: Regex::new(r"(?i)^yang\b").unwrap(),
                },
                Rule {
                    name: consts::YIN,
                    re: Regex::new(r"(?i)^yin\b").unwrap(),
                },
                // --- символы ---
                Rule {
                    name: consts::COMMA,
                    re: Regex::new(r"^,").unwrap(),
                },
                Rule {
                    name: consts::DOT,
                    re: Regex::new(r"^\.").unwrap(),
                },
                Rule {
                    name: consts::LBRACE,
                    re: Regex::new(r"^\{").unwrap(),
                },
                Rule {
                    name: consts::RBRACE,
                    re: Regex::new(r"^\}").unwrap(),
                },
                // --- числа ---
                Rule {
                    name: consts::INT,
                    re: Regex::new(r"^-?\d+").unwrap(),
                },
                Rule {
                    name: consts::FLOAT,
                    re: Regex::new(r"^[0-9][0-9A-Za-z\-]*(?:\.[0-9A-Za-z\-]+)*").unwrap(),
                },
                // --- строка ---
                Rule {
                    name: consts::QUOTE,
                    re: Regex::new(r#"^""#).unwrap(),
                },
                Rule {
                    name: consts::STR_MODE,
                    re: Regex::new(r"$^").unwrap(),
                },
                // --- переменные ---
                Rule {
                    name: consts::VAR,
                    re: Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap(),
                },
            ],
            fictive_re: Regex::new(r"^").unwrap(),
            // err: err_h,
            cur_span: None,
            is_string: false,
            is_opened_id: false,
        }
    }

    fn gen_token(&self, tt: TokenType, val: TokenVal) -> Token {
        if let Some(s) = self.cur_span {
            return Token {
                token_type: tt,
                val: val,
                span: s,
            };
        }

        unreachable!()
    }

    fn read_str(&mut self, code: &str, offset: usize, pos: &PosIndex, id: FileId) -> String {
        let start = offset;
        let mut end = start;
        let mut is_esc = false;
        let mut res = String::new();

        while let Some(ch) = code[end..].chars().next() {
            match ch {
                '\\' => is_esc = !is_esc,
                '"' => {
                    if !is_esc {
                        self.cur_span =
                            Some(pos.range_to_span(id, start.saturating_sub(1), end + 1));
                        return res;
                    }
                }

                _ => is_esc = false,
            }

            end += ch.len_utf8();
            res.push(ch);
        }

        panic!(
            "unterminated string! text after opening quote: \n{}",
            code[start..].to_string()
        )
    }

    pub fn lex_code(&mut self, id: FileId, sm: &SourceManager, posindx: &PosIndex) -> Result<Vec<Token>, Error> {
        let mut pos: usize = 0;
        let mut tokens: Vec<Token> = Vec::new();
        let code = sm.get_file_src(id);

        while pos < code.len() {
            let rem = &code[pos..];

            let mut best_match: Option<(&Rule, Match)> = None;

            for rule in &self.rules {
                if let Some(m) = rule.re.find(rem) {
                    if m.start() != 0 {
                        continue;
                    }

                    match &best_match {
                        None => best_match = Some((rule, m)),
                        Some((_, prev)) => {
                            if m.len() > prev.len() {
                                best_match = Some((rule, m))
                            }
                        }
                    }
                }
            }

            if self.is_string {
                best_match = Some((
                    self.rules.iter().find(|r| r.name == consts::STR_MODE).unwrap(),
                    self.fictive_re.find(rem).unwrap(),
                ))
            }

            if let Some((rule, m)) = best_match {
                let mut m_len = m.len();
                let m_content = &rem[..m.end()];
                self.cur_span = Some(posindx.range_to_span(id, pos, m.end() + pos));

                // дальше сверка с правилами

                println!("matched token {}", rule.name);

                match rule.name {
                    consts::NEWLINE => tokens.push(self.gen_token(NEWLINE, TokenVal::None)),
                    consts::WHITESPACE => {
                        tokens.push(self.gen_token(TokenType::WHITESPACE, TokenVal::None));
                    }

                    consts::TO => tokens.push(self.gen_token(TokenType::TO, TokenVal::None)),
                    consts::STUDY => tokens.push(self.gen_token(TokenType::STUDY, TokenVal::None)),
                    consts::THE => tokens.push(self.gen_token(TokenType::THE, TokenVal::None)),
                    consts::RITUAL => tokens.push(self.gen_token(TokenType::RITUAL, TokenVal::None)),
                    consts::OF => tokens.push(self.gen_token(TokenType::OF, TokenVal::None)),
                    consts::WITH => tokens.push(self.gen_token(TokenType::WITH, TokenVal::None)),
                    consts::IS => tokens.push(self.gen_token(TokenType::IS, TokenVal::None)),
                    consts::A => tokens.push(self.gen_token(TokenType::A, TokenVal::None)),
                    consts::PLEASURE => tokens.push(self.gen_token(TokenType::PLEASURE, TokenVal::None)),

                    consts::LET => tokens.push(self.gen_token(TokenType::LET, TokenVal::None)),
                    consts::US => tokens.push(self.gen_token(TokenType::US, TokenVal::None)),
                    consts::CONSULT => tokens.push(self.gen_token(TokenType::CONSULT, TokenVal::None)),
                    consts::WISDOM => tokens.push(self.gen_token(TokenType::WISDOM, TokenVal::None)),
                    consts::REGRADING => tokens.push(self.gen_token(TokenType::REGRADING, TokenVal::None)),

                    consts::BEGINNING => tokens.push(self.gen_token(TokenType::BEGINNING, TokenVal::None)),
                    consts::DEFINITION => tokens.push(self.gen_token(TokenType::DEFINITION, TokenVal::None)),
                    consts::TERMS => tokens.push(self.gen_token(TokenType::TERMS, TokenVal::None)),

                    consts::HE => tokens.push(self.gen_token(TokenType::HE, TokenVal::None)),
                    consts::FIRST => tokens.push(self.gen_token(TokenType::FIRST, TokenVal::None)),
                    consts::PRESENTS => tokens.push(self.gen_token(TokenType::PRESENTS, TokenVal::None)),
                    consts::HIS => tokens.push(self.gen_token(TokenType::HIS, TokenVal::None)),
                    consts::WORDS => tokens.push(self.gen_token(TokenType::WORDS, TokenVal::None)),
                    consts::AS => tokens.push(self.gen_token(TokenType::AS, TokenVal::None)),
                    consts::THEN => tokens.push(self.gen_token(TokenType::THEN, TokenVal::None)),
                    consts::ACCORDING => tokens.push(self.gen_token(TokenType::ACCORDING, TokenVal::None)),
                    consts::THEM => tokens.push(self.gen_token(TokenType::THEM, TokenVal::None)),
                    consts::ACTS => tokens.push(self.gen_token(TokenType::ACTS, TokenVal::None)),

                    consts::WHO => tokens.push(self.gen_token(TokenType::WHO, TokenVal::None)),
                    consts::BY => tokens.push(self.gen_token(TokenType::BY, TokenVal::None)),
                    consts::REANIMATING => tokens.push(self.gen_token(TokenType::REANIMATING, TokenVal::None)),
                    consts::CAN => tokens.push(self.gen_token(TokenType::CAN, TokenVal::None)),
                    consts::GAIN => tokens.push(self.gen_token(TokenType::GAIN, TokenVal::None)),
                    consts::KNOWLEGE => tokens.push(self.gen_token(TokenType::KNOWLEGE, TokenVal::None)),

                    consts::HARMONIZED => tokens.push(self.gen_token(TokenType::HARMONIZED, TokenVal::None)),
                    consts::DIMINISHED => tokens.push(self.gen_token(TokenType::DIMINISHED, TokenVal::None)),
                    consts::MULTIPLED => tokens.push(self.gen_token(TokenType::MULTIPLED, TokenVal::None)),
                    consts::SHARED => tokens.push(self.gen_token(TokenType::SHARED, TokenVal::None)),
                    consts::AMONG => tokens.push(self.gen_token(TokenType::AMONG, TokenVal::None)),
                    consts::RAISED => tokens.push(self.gen_token(TokenType::RAISED, TokenVal::None)),
                    consts::POWER => tokens.push(self.gen_token(TokenType::POWER, TokenVal::None)),
                    consts::PARTITIONED => tokens.push(self.gen_token(TokenType::PARTITIONED, TokenVal::None)),
                    consts::REMAINDER => tokens.push(self.gen_token(TokenType::REMAINDER, TokenVal::None)),

                    consts::AND => tokens.push(self.gen_token(TokenType::AND, TokenVal::None)),

                    consts::YANG => tokens.push(self.gen_token(TokenType::BOOL, TokenVal::Bool(true))),
                    consts::YIN => tokens.push(self.gen_token(TokenType::BOOL, TokenVal::Bool(false))),

                    consts::COMMA => tokens.push(self.gen_token(TokenType::COMMA, TokenVal::None)),
                    consts::DOT => tokens.push(self.gen_token(TokenType::DOT, TokenVal::None)),
                    consts::LBRACE => tokens.push(self.gen_token(TokenType::LBRACE, TokenVal::None)),
                    consts::RBRACE => tokens.push(self.gen_token(TokenType::RBRACE, TokenVal::None)),

                    consts::FLOAT => {
                        let val = match m_content.parse::<f64>() {
                            Ok(v) => {
                                if v.is_infinite() {
                                    panic!()
                                }
                                if v.is_nan() {
                                    panic!()
                                }
                                v
                            }

                            Err(_) => {
                                panic!("value {}, is not floatable", m_content)
                            } // value_err
                        };

                        tokens.push(self.gen_token(TokenType::FLOAT, TokenVal::Float(val)));
                    }

                    consts::INT => {
                        let val = match m_content.parse::<i64>() {
                            Ok(v) => v,
                            Err(e) => match e.kind() {
                                IntErrorKind::InvalidDigit => panic!(),
                                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => panic!(),
                                _ => unreachable!(),
                            },
                        };

                        tokens.push(self.gen_token(TokenType::INT, TokenVal::Int(val)));
                    }

                    consts::QUOTE => self.is_string = true,
                    consts::STR_MODE => {
                        self.is_string = false;
                        let str = self.read_str(code, pos, &posindx, id);

                        m_len = str.len() + '"'.len_utf8();

                        tokens.push(self.gen_token(TokenType::STR, TokenVal::Str(str)));
                    }

                    consts::VAR => {
                        if let Some(f) = m_content.chars().next() {
                            if f.is_numeric() || f == '_' {
                                panic!()
                            }
                        }

                        tokens.push(self.gen_token(TokenType::VAR, TokenVal::Var(m_content.to_string())));
                    }

                    _ => unreachable!("The rule {} is not matchable", rule.name),
                }

                pos += m_len;
            } else {
                panic!("offset: {}, rem: {}", pos, rem)
            }
        }

        self.cur_span = None;
        self.is_opened_id = false;
        self.is_string = false;

        Ok(tokens)
    }
}
