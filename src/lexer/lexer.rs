use core::f64;
use std::{collections::VecDeque, num::IntErrorKind, vec};
use crate::{consts::{self}, errors::{ErrHandler, Error}, file_manager::manager::FileManager, span::{self, PosIndex, Span}};
use regex::{Captures, Match, Regex};
use super::{structs::{Token, TokenType}};

pub struct Rule {
    name: &'static str,
    re: Regex,
}

pub struct Lexer<'a> {
    rules: Vec<Rule>,
    fictive_re: Regex,
    // err: ErrHandler,
    cur_span: Option<Span>,
    is_string: bool,
    is_opened_id: bool,
    fm: &'a FileManager
}


impl Lexer<'_> {

    pub fn new(manager: FileManager) -> Self {

        Self {
            rules: vec![

                // whitespace
                Rule { name: consts::NEWLINE, re: Regex::new(r"^(?:\r\n|\n|\r)").unwrap(),},
                Rule { name: consts::WHITESPACE, re: Regex::new(r"^[ \t]+").unwrap() },

                // func_def
                Rule { name: consts::TO, re: Regex::new(r"(?i)^To\b").unwrap() },
                Rule { name: consts::STUDY, re: Regex::new(r"(?i)^study\b").unwrap() },
                Rule { name: consts::THE, re: Regex::new(r"(?i)^the\b").unwrap() },
                Rule { name: consts::RITUAL, re: Regex::new(r"(?i)^ritual\b").unwrap() },
                Rule { name: consts::OF, re: Regex::new(r"(?i)^of\b").unwrap() },
                Rule { name: consts::WITH, re: Regex::new(r"(?i)^with\b").unwrap() },
                Rule { name: consts::IS, re: Regex::new(r"(?i)^is\b").unwrap() },
                Rule { name: consts::A, re: Regex::new(r"(?i)^a\b").unwrap() },
                Rule { name: consts::PLEASURE, re: Regex::new(r"(?i)^pleasure\b").unwrap() },

                // func_call
                Rule { name: consts::LET, re: Regex::new(r"(?i)^let\b").unwrap() },
                Rule { name: consts::US,  re: Regex::new(r"(?i)^us\b").unwrap() },
                Rule { name: consts::CONSULT, re: Regex::new(r"(?i)^consult\b").unwrap() },
                Rule { name: consts::WISDOM, re: Regex::new(r"(?i)^wisdom\b").unwrap() },
                Rule { name: consts::REGRADING, re: Regex::new(r"(?i)^regarding\b").unwrap() },

                // return
                Rule { name: consts::BEGINNING, re: Regex::new(r"(?i)^beginning\b").unwrap() },
                Rule { name: consts::DEFINITION, re: Regex::new(r"(?i)^definition\b").unwrap() },
                Rule { name: consts::TERMS, re: Regex::new(r"(?i)^terms\b").unwrap() },

                // print
                Rule { name: consts::HE,        re: Regex::new(r"(?i)^he\b").unwrap() },
                Rule { name: consts::FIRST,     re: Regex::new(r"(?i)^first\b").unwrap() },
                Rule { name: consts::PRESENTS,  re: Regex::new(r"(?i)^presents\b").unwrap() },
                Rule { name: consts::HIS,       re: Regex::new(r"(?i)^his\b").unwrap() },
                Rule { name: consts::WORDS,     re: Regex::new(r"(?i)^words\b").unwrap() },
                Rule { name: consts::AS,        re: Regex::new(r"(?i)^as\b").unwrap() },
                Rule { name: consts::THEN,      re: Regex::new(r"(?i)^then\b").unwrap() },
                Rule { name: consts::ACCORDING, re: Regex::new(r"(?i)^according\b").unwrap() },
                Rule { name: consts::THEM,      re: Regex::new(r"(?i)^them\b").unwrap() },
                Rule { name: consts::ACTS,      re: Regex::new(r"(?i)^acts\b").unwrap() },

                // --- assign ---
                Rule { name: consts::WHO,          re: Regex::new(r"(?i)^who\b").unwrap() },
                Rule { name: consts::BY,           re: Regex::new(r"(?i)^by\b").unwrap() },
                Rule { name: consts::REANIMATING,  re: Regex::new(r"(?i)^reanimating\b").unwrap() },
                Rule { name: consts::CAN,          re: Regex::new(r"(?i)^can\b").unwrap() },
                Rule { name: consts::GAIN,         re: Regex::new(r"(?i)^gain\b").unwrap() },
                Rule { name: consts::KNOWLEGE,     re: Regex::new(r"(?i)^knowledge\b").unwrap() },

                // --- операторы (по словам) ---
                Rule { name: consts::HARMONIZED,  re: Regex::new(r"(?i)^harmonized\b").unwrap() },
                Rule { name: consts::DIMINISHED,  re: Regex::new(r"(?i)^diminished\b").unwrap() },
                Rule { name: consts::MULTIPLED,   re: Regex::new(r"(?i)^multiplied\b").unwrap() },
                Rule { name: consts::SHARED,      re: Regex::new(r"(?i)^shared\b").unwrap() },
                Rule { name: consts::AMONG,       re: Regex::new(r"(?i)^among\b").unwrap() },
                Rule { name: consts::RAISED,      re: Regex::new(r"(?i)^raised\b").unwrap() },
                Rule { name: consts::POWER,       re: Regex::new(r"(?i)^power\b").unwrap() },
                Rule { name: consts::PARTITIONED, re: Regex::new(r"(?i)^partitioned\b").unwrap() },
                Rule { name: consts::REMAINDER,   re: Regex::new(r"(?i)^remainder\b").unwrap() },

                // --- служебные ---
                Rule { name: consts::AND, re: Regex::new(r"(?i)^and\b").unwrap() },

                // --- bool ---
                Rule { name: consts::YANG, re: Regex::new(r"(?i)^yang\b").unwrap() },
                Rule { name: consts::YIN,  re: Regex::new(r"(?i)^yin\b").unwrap() },

                // --- символы ---
                Rule { name: consts::COMMA,  re: Regex::new(r"^,").unwrap() },
                Rule { name: consts::DOT,    re: Regex::new(r"^\.").unwrap() },
                Rule { name: consts::LBRACE, re: Regex::new(r"^\{").unwrap() },
                Rule { name: consts::RBRACE, re: Regex::new(r"^\}").unwrap() },

                // --- числа ---
                Rule { name: consts::INT,   re: Regex::new(r"^-?\d+").unwrap() },
                Rule { name: consts::FLOAT, re: Regex::new(r"^[0-9][0-9A-Za-z\-]*(?:\.[0-9A-Za-z\-]+)*").unwrap() },

                // --- строка ---
                Rule { name: consts::QUOTE, re: Regex::new(r#"^""#).unwrap() },
                Rule { name: consts::STR, re: Regex::new(r"$^").unwrap(),},

                // --- переменные ---
                Rule { name: consts::VAR, re: Regex::new(r"^[A-Za-z][A-Za-z0-9]*").unwrap() },

            ],
            fictive_re: Regex::new(r"^").unwrap(),
            // err: err_h,
            cur_span: None,
            is_string: false,
            is_opened_id: false,
            fm: &manager

        }
        
    }


    fn gen_token(&self, tt: TokenType) -> Token {

        if let Some(s) = self.cur_span {

            return Token { token_type: tt, span: s };
            
        }

        unreachable!()
        
    }


    fn read_str(&mut self, code: &str, offset: usize) -> String {

        let start = offset;
        let mut end = start;
        let mut is_esc = false;
        let mut res = String::new();

        while let Some(ch) = code[end..].chars().next() {

            match ch {

                '\\' => is_esc = !is_esc,
                '"' => if !is_esc {

                    self.cur_span = Some(self.pos.range_to_span(start.saturating_sub(1), end + 1));
                    return res;
                        
                },

                _ => is_esc = false,
                
            }

            end += ch.len_utf8();
            res.push(ch);
            
        }

        panic!("unterminated string! text after openong quote: \n{}", code[start..].to_string())
        
    }


    pub fn lex_code(&mut self, code: &str) -> Result<Vec<Token>, Error> {

        let mut pos: usize = 0;
        let mut tokens: Vec<Token> = Vec::new();

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

                best_match = Some((self.rules.iter().find(|r| r.name == consts::STR).unwrap(), self.fictive_re.find(rem).unwrap()))
                
            }

            if let Some((rule, m)) = best_match {

                let mut m_len = m.len();
                let m_content = &rem[..m.end()];
                self.cur_span = Some(self.pos.range_to_span(pos, m.end() + pos));

                // дальше сверка с правилами

                println!("matched token {}", rule.name);

                match rule.name {

                    consts::NEWLINE => tokens.push(self.gen_token(TokenType::NEWLINE)),
                    consts::WHITESPACE => {tokens.push(self.gen_token(TokenType::WHITESPACE));},

                    consts::TO => tokens.push(self.gen_token(TokenType::TO)),
                    consts::STUDY => tokens.push(self.gen_token(TokenType::STUDY)),
                    consts::THE => tokens.push(self.gen_token(TokenType::THE)),
                    consts::RITUAL => tokens.push(self.gen_token(TokenType::RITUAL)),
                    consts::OF => tokens.push(self.gen_token(TokenType::OF)),
                    consts::WITH => tokens.push(self.gen_token(TokenType::WITH)),
                    consts::IS => tokens.push(self.gen_token(TokenType::IS)),
                    consts::A => tokens.push(self.gen_token(TokenType::A)),
                    consts::PLEASURE => tokens.push(self.gen_token(TokenType::PLEASURE)),


                    consts::LET => tokens.push(self.gen_token(TokenType::LET)),
                    consts::US => tokens.push(self.gen_token(TokenType::US)),
                    consts::CONSULT => tokens.push(self.gen_token(TokenType::CONSULT)),
                    consts::WISDOM => tokens.push(self.gen_token(TokenType::WISDOM)),
                    consts::REGRADING => tokens.push(self.gen_token(TokenType::REGRADING)),


                    consts::BEGINNING => tokens.push(self.gen_token(TokenType::BEGINNING)),
                    consts::DEFINITION => tokens.push(self.gen_token(TokenType::DEFINITION)),
                    consts::TERMS => tokens.push(self.gen_token(TokenType::TERMS)),


                    consts::HE => tokens.push(self.gen_token(TokenType::HE)),
                    consts::FIRST => tokens.push(self.gen_token(TokenType::FIRST)),
                    consts::PRESENTS => tokens.push(self.gen_token(TokenType::PRESENTS)),
                    consts::HIS => tokens.push(self.gen_token(TokenType::HIS)),
                    consts::WORDS => tokens.push(self.gen_token(TokenType::WORDS)),
                    consts::AS => tokens.push(self.gen_token(TokenType::AS)),
                    consts::THEN => tokens.push(self.gen_token(TokenType::THEN)),
                    consts::ACCORDING => tokens.push(self.gen_token(TokenType::ACCORDING)),
                    consts::THEM => tokens.push(self.gen_token(TokenType::THEM)),
                    consts::ACTS => tokens.push(self.gen_token(TokenType::ACTS)),


                    consts::WHO => tokens.push(self.gen_token(TokenType::WHO)),
                    consts::BY => tokens.push(self.gen_token(TokenType::BY)),
                    consts::REANIMATING => tokens.push(self.gen_token(TokenType::REANIMATING)),
                    consts::CAN => tokens.push(self.gen_token(TokenType::CAN)),
                    consts::GAIN => tokens.push(self.gen_token(TokenType::GAIN)),
                    consts::KNOWLEGE => tokens.push(self.gen_token(TokenType::KNOWLEGE)),


                    consts::HARMONIZED => tokens.push(self.gen_token(TokenType::HARMONIZED)),
                    consts::DIMINISHED => tokens.push(self.gen_token(TokenType::DIMINISHED)),
                    consts::MULTIPLED => tokens.push(self.gen_token(TokenType::MULTIPLED)),
                    consts::SHARED => tokens.push(self.gen_token(TokenType::SHARED)),
                    consts::AMONG => tokens.push(self.gen_token(TokenType::AMONG)),
                    consts::RAISED => tokens.push(self.gen_token(TokenType::RAISED)),
                    consts::POWER => tokens.push(self.gen_token(TokenType::POWER)),
                    consts::PARTITIONED => tokens.push(self.gen_token(TokenType::PARTITIONED)),
                    consts::REMAINDER => tokens.push(self.gen_token(TokenType::REMAINDER)),


                    consts::AND => tokens.push(self.gen_token(TokenType::AND)),


                    consts::YANG => tokens.push(self.gen_token(TokenType::BOOL(true))),
                    consts::YIN => tokens.push(self.gen_token(TokenType::BOOL(false))),


                    consts::COMMA => tokens.push(self.gen_token(TokenType::COMMA)),
                    consts::DOT => tokens.push(self.gen_token(TokenType::DOT)),
                    consts::LBRACE => tokens.push(self.gen_token(TokenType::LBRACE)),
                    consts::RBRACE => tokens.push(self.gen_token(TokenType::RBRACE)),


                    consts::FLOAT => {

                        let val = match m_content.parse::<f64>() {

                            Ok(v) => {
                                if v.is_infinite() {
                                    panic!()
                                } if v.is_nan() {
                                    panic!()
                                }
                                v
                            },

                            Err(_) => {panic!("value {}, is not floatable", m_content)}// value_err
                            
                        };

                        tokens.push(self.gen_token(TokenType::FLOAT(val)));

                    },


                    consts::INT => {

                        let val = match m_content.parse::<i64>() {

                            Ok(v) => v,
                            Err(e) => match e.kind() {

                                IntErrorKind::InvalidDigit => panic!(),
                                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => panic!(),
                                _ => unreachable!()
                                
                            }
                            
                        };

                        tokens.push(self.gen_token(TokenType::INT(val)));

                    },


                    consts::QUOTE => self.is_string = true,
                    consts:: STR => {

                        self.is_string = false;
                        let str = self.read_str(code, pos);

                        m_len = str.len() + '"'.len_utf8();

                        tokens.push(self.gen_token(TokenType::STR(str)));


                    },


                    consts::VAR => {

                        if let Some(f) = m_content.chars().next() {

                            if f.is_numeric() || f == '_' {

                                panic!()
                                
                            }
                            
                        }

                        tokens.push(self.gen_token(TokenType::VAR(m_content.to_string())));

                    },


                    _ => unreachable!("The rule {} is not matchable", rule.name)

                    
                }

                pos += m_len;
                
            } else {

                panic!("offset: {}, rem: {}", pos, rem)
                
            }

            }

            Ok(tokens)
            
        }

        
        
    }
    

    // "abc"