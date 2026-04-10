use std::{collections::{HashMap, btree_map::Values, hash_map}, ffi::CStr, vec};
use crate::errors::ErrHandler;
use regex::{Captures, Regex, bytes::CaptureMatches};

use crate::consts;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BLOCK_START,
    BLOCK_END,
    NEWEXPR,
    NEWLINE,
    COMMA,

    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EXPONENTIATION,
    DIV_WITH_REMAIN,
    REMAIN_FROM_DIV,

    ASSIGN,
    FUNC_DEF(String),
    FUNC_VOID(String),
    FUNC_CALL(String),
    RETURN(String),
    PRINT,
    INPUT,
    VAR(String),
    INT(i64),
    FLOAT(f64),
    STR(String),
    BOOL(bool),
}

// другие структуры

struct Rule {
    name: &'static str,
    re: Regex,
}

pub struct Lexer {
    rules: Vec<Rule>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Rule { 
                    name: consts::FUNC_DEF, 
                    re: Regex::new(r"^To study the ritual of \[\s*(?P<name>\w+)\s*\] with \[\s*(?P<args>.*)\s*\] is a pleasure\.").unwrap() 
                },
                Rule { 
                    name: consts::FUNC_VOID, 
                    re: Regex::new(r"^The gentleman applies himself to the root, for \[\s*(?P<name>\w+)\s*\] and its \[\s*(?P<args>.*)\s*\] shall be made manifest\.").unwrap() 
                },
                Rule {
                    name: consts::FUNC_CALL, 
                    re: Regex::new(r"^Let us consult the wisdom of \[\s*(?P<name>\w+)\s*\] regarding \[\s*(?P<args>.*)\s*\]\.").unwrap()
                },
                Rule {
                    name: consts::RETURN,
                    re: Regex::new(r"^The beginning of wisdom is the definition of terms \[\s*(?P<val>.*)\s*\]\.").unwrap(),
                },
                Rule { 
                    name: consts::PRINT, 
                    re: Regex::new(r"^He first presents his words as \[\s*(?P<val>.*)\s*\], and then according to them he acts\.").unwrap() 
                },
                Rule { 
                    name: consts::ASSIGN, 
                    re: Regex::new(r"^He who by reanimating \[\s*(?P<val>.*)\s*\] can gain knowledge of \[\s*(?P<var>.*)\s*\]\.").unwrap() 
                },

                Rule {
                    name: consts::PLUS, 
                    re: Regex::new(r"^\s*harmonized with\s*").unwrap()
                },
                Rule {
                    name: consts::MINUS, 
                    re: Regex::new(r"^\s*diminished by\s*").unwrap()
                },
                Rule {
                    name: consts::MULTIPLY, 
                    re: Regex::new(r"^\s*multiplied by\s*").unwrap()
                },
                Rule {
                    name: consts::DIVIDE, 
                    re: Regex::new(r"^\s*shared among\s*").unwrap()
                },
                Rule {
                    name: consts::EXPONENTIATION, 
                    re: Regex::new(r"^\s*raised to the power of\s*").unwrap()
                },
                Rule {
                    name: consts::DIV_WITH_REMAIN, 
                    re: Regex::new(r"^\s*partitioned by\s*").unwrap()
                },
                Rule {
                    name: consts::REMAIN_FROM_DIV, 
                    re: Regex::new(r"^\s*with a remainder of\s*").unwrap()
                },
                Rule {
                    name: consts::FLOAT, 
                    re: Regex::new(r#"^\s*(?P<val>-?\d+\.\d+)\s*"#).unwrap()},
                Rule {
                    name: consts::INT, 
                    re: Regex::new(r"^\s*(?P<val>-?\d+)\s*").unwrap() },
                Rule {
                    name: consts::STR, 
                    re: Regex::new(r#"^\s*"(?P<val>.*?)"\s*"#).unwrap()},
                Rule {
                    name: consts::BOOL, 
                    re: Regex::new(r"^\s*(?P<val>Yang|Yin)\b").unwrap()
                },

                Rule {
                    name: consts::BLOCK_START,
                    re: Regex::new(r"^\{").unwrap(),
                },
                Rule {
                    name: consts::BLOCK_END,
                    re: Regex::new(r"^\}").unwrap(),
                },
                Rule {
                    name: consts::COMMA,
                    re: Regex::new(r"^,").unwrap(),
                },

                Rule {
                    name: consts::VAR, 
                    re: Regex::new(r"^\s*(?P<var>[a-zA-Z_]\w*)").unwrap()
            },
            ],
        }
    }

    

    pub fn parse_code(&self, code: &str, err_h: ErrHandler, is_inner: bool, main_line: usize) -> Vec<Token> {
        let mut tokens:Vec<Token> = Vec::new();
        let lines = consts::prepare_code(&code);
        let mut line_index=0;
        let mut cur_pos_in_line:usize = 0;
        let mut main_line = main_line;
        let err_handler = err_h.clone();

        while line_index < lines.len() {
            let line = lines[line_index];
            let remaining = &line[cur_pos_in_line..];

            if remaining.is_empty() || remaining.starts_with("//") {
                if !is_inner {
                    tokens.push(Token::NEWLINE);
                }
                line_index += 1;
                main_line += 1;
                cur_pos_in_line = 0;
                continue;
            };

            

            let mut earl_macth: Option<(&Rule, Captures, usize)> = None;

            for rule in &self.rules {
                if let Some(caps) = rule.re.captures(&remaining) {
                    let start_pos = caps.get(0).unwrap().start();

                    if earl_macth.as_ref().map_or(true, |(_, _, pos)| start_pos < *pos) {
                        earl_macth = Some((rule, caps, start_pos));
                    }
                }
            }

            if let Some((rule, caps, start_pos)) = earl_macth {
                if start_pos > 0 {
                    err_handler.syntax_err(main_line);
                }

                let matched_str = caps.get(0).unwrap().as_str();
                let matched_len = matched_str.len();

                match rule.name {
                
                consts::FUNC_DEF => {
                    let name = &caps["name"];
                    let args = self.parse_code(&caps["args"], err_h.clone(), true, main_line);
                    
                    tokens.push(Token::FUNC_DEF(name.to_string()));

                    tokens.push(Token::BLOCK_START);
                    tokens.extend(args);
                    tokens.push(Token::BLOCK_END);
                    tokens.push(Token::NEWEXPR);

                    
                },

                consts::FUNC_VOID => {
                    let name = &caps["name"];
                    let args = self.parse_code(&caps["args"], err_h.clone(), true, main_line);

                    tokens.push(Token::FUNC_VOID(name.to_string()));
                    tokens.push(Token::BLOCK_START);
                    tokens.extend(args);
                    tokens.push(Token::BLOCK_END);
                    tokens.push(Token::NEWEXPR);
                    
                    
                },

                consts::FUNC_CALL => {
                    let name = &caps["name"];
                    let args = self.parse_code(&caps["args"], err_h.clone(), true, main_line);

                    tokens.push(Token::FUNC_CALL(name.to_string()));
                    tokens.push(Token::BLOCK_START);
                    tokens.extend(args);
                    tokens.push(Token::BLOCK_END);

                    
                    
                },

                consts::RETURN => {
                    tokens.push(Token::RETURN(caps["val"].to_string()));
                    tokens.push(Token::NEWEXPR);
                    
                },

                consts::PRINT => {
                    let vals = self.parse_code(&caps["val"], err_h.clone(), true, main_line);

                    tokens.push(Token::PRINT);

                    tokens.push(Token::BLOCK_START);
                    tokens.extend(vals);
                    tokens.push(Token::BLOCK_END);
                    tokens.push(Token::NEWEXPR);

                    
                },

                consts::ASSIGN => {
                    let val = self.parse_code(&caps["val"], err_h.clone(), true, main_line);
                    let var = self.parse_code(&caps["var"], err_h.clone(), true, main_line);

                    tokens.push(Token::ASSIGN);
                    tokens.push(Token::BLOCK_START);
                    tokens.extend(var);
                    tokens.push(Token::BLOCK_END);

                    tokens.push(Token::BLOCK_START);
                    tokens.extend(val);
                    tokens.push(Token::BLOCK_END);
                    tokens.push(Token::NEWEXPR);

                    
                },

                consts::PLUS => {
                    tokens.push(Token::PLUS);
                    
                },

                consts::MINUS => {
                    tokens.push(Token::MINUS);
                    
                },

                consts::MULTIPLY => {
                    tokens.push(Token::MULTIPLY);
                    
                },

                consts::DIVIDE => {
                    tokens.push(Token::DIVIDE);
                    
                },

                consts::EXPONENTIATION => {
                    tokens.push(Token::EXPONENTIATION);
                    
                },

                consts::DIV_WITH_REMAIN => {
                    tokens.push(Token::DIV_WITH_REMAIN);
                    
                },

                consts::REMAIN_FROM_DIV => {
                    tokens.push(Token::REMAIN_FROM_DIV);
                    
                },

                consts::FLOAT => {
                    let val = &caps["val"];

                    match val.parse::<f64>() {
                        Ok(v) => {
                            tokens.push(Token::FLOAT(v));
                        },

                        Err(_) => {err_handler.value_overflow_err(main_line);}
                    };
                    
                },

                consts::INT => {
                    let val = &caps["val"];

                    match val.parse::<i64>() {
                        Ok(v) => {
                            tokens.push(Token::INT(v));
                        },

                        Err(_) => {err_handler.value_overflow_err(main_line);}
                    };
                    
                },

                consts::STR => {
                    let val = &caps["val"];
                    tokens.push(Token::STR(val.to_string()));
                }

                consts::BOOL => {
                    let val = &caps["val"] == consts::TRUE;
                    tokens.push(Token::BOOL(val));
                }

                consts::BLOCK_START => {
                        tokens.push(Token::BLOCK_START);
                        
                },

                consts::BLOCK_END => {
                    tokens.push(Token::BLOCK_END);
                    
                },

                consts::COMMA => {
                    tokens.push(Token::COMMA);
                },

                consts::VAR => {
                let var = &caps["var"];

                    tokens.push(Token::VAR(var.to_string()));
                }
                
                _ => {
                    err_handler.token_not_found_err(main_line);
                }
                }
                cur_pos_in_line += matched_len;
            } else {
                err_handler.syntax_err(main_line);
            }
        }

            return tokens;
        }
        }
