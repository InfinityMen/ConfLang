// use std::{collections::VecDeque, num::IntErrorKind, vec};
// use crate::{consts, errors::{ErrHandler, enums::ErrType}};
// use regex::{Captures, Regex};
// use super::{structs::{Token, TokenType}};

// pub struct Rule {
//     name: &'static str,
//     re: Regex,
// }

// pub struct Lexer {
//     rules: Vec<Rule>,
//     debug: bool,
//     is_quote: bool,

//     err_handler: ErrHandler,
//     line: usize,
//     column: usize,
// }

// impl Lexer {
//     pub fn new(err_h: ErrHandler, is_debug: bool) -> Self {
//         Self { rules: vec![
//             Rule { 
//                     name: consts::FUNC_DEF, 
//                     re: Regex::new(r"^To study the ritual of \[\s*(?P<name>\w+)\s*\] with \[\s*(?P<args>[^\]]*)\s*\] is a pleasure\.").unwrap() 
//                 },
//                 Rule {
//                     name: consts::FUNC_CALL, 
//                     re: Regex::new(r"^Let us consult the wisdom of \[\s*(?P<name>\w+)\s*\] regarding \[\s*(?P<args>[^\]]*)\s*\]\.").unwrap()
//                 },
//                 Rule {
//                     name: consts::RETURN,
//                     re: Regex::new(r"^The beginning of wisdom is the definition of terms \[\s*(?P<val>[^\]]*)\s*\]\.").unwrap(),
//                 },
//                 Rule { 
//                     name: consts::PRINT, 
//                     re: Regex::new(r"^He first presents his words as \[\s*(?P<val>[^\]]*)\s*\], and then according to them he acts\.").unwrap() 
//                 },
//                 Rule { 
//                     name: consts::ASSIGN, 
//                     re: Regex::new(r"^He who by reanimating \[\s*(?P<val>[^\]]*)\s*\] can gain knowledge of \[\s*(?P<var>[^\]]*)\s*\]\.").unwrap() 
//                 },

//                 Rule {
//                     name: consts::PLUS, 
//                     re: Regex::new(r"^\s*harmonized with\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::MINUS, 
//                     re: Regex::new(r"^\s*diminished by\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::MULTIPLY, 
//                     re: Regex::new(r"^\s*multiplied by\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::DIVIDE, 
//                     re: Regex::new(r"^\s*shared among\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::EXPONENTIATION, 
//                     re: Regex::new(r"^\s*raised to the power of\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::DIV_WITH_REMAIN, 
//                     re: Regex::new(r"^\s*partitioned by\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::REMAIN_FROM_DIV, 
//                     re: Regex::new(r"^\s*with a remainder of\s*").unwrap()
//                 },
//                 Rule {
//                     name: consts::FLOAT, 
//                     re: Regex::new(r"^\s*(?P<val>[+\-.\d]+)(?:\s|,|\]|$)").unwrap()},
//                 Rule {
//                     name: consts::INT, 
//                     re: Regex::new(r"^\s*(?P<val>[-\d]+)(?:\s|,|\]|$)").unwrap() },
//                 Rule {
//                     name: consts::BOOL, 
//                     re: Regex::new(r"^\s*(?P<val>Yang|Yin)\b").unwrap()
//                 },

//                 Rule {
//                     name: consts::BLOCK_START,
//                     re: Regex::new(r"^\{").unwrap(),
//                 },
//                 Rule {
//                     name: consts::BLOCK_END,
//                     re: Regex::new(r"^\}").unwrap(),
//                 },
//                 Rule {
//                     name: consts::COMMA,
//                     re: Regex::new(r"^,").unwrap(),
//                 },

//                 Rule {
//                     name: consts::VAR, 
//                     re: Regex::new(r"^\s*(?P<var>[A-Za-z][A-Za-z0-9_]*)").unwrap()
//             },
//         ], debug:is_debug, is_quote:false, err_handler: err_h, line: 0, column: 0 }
//     }

//     fn gen_token(&mut self, t_type: TokenType) -> Token {
//         return Token { token_type: t_type, line: self.line, column: self.column };
//     }

//     fn read_string(&mut self, code: &str) -> (String, String) {

//         let mut backslash_count: usize = 0;
//         let mut res = String::new();
//         let mut chrs: VecDeque<char> = code.chars().collect();
        
//             while let Some(ch) = chrs.pop_front() {

//                 match ch {
                    
//                     '\\' => {
//                         backslash_count += 1;
//                     },

//                     '"' => {
//                         if backslash_count % 2 == 0 {
//                             return (res, chrs.into_iter().collect::<String>());
//                         }
//                         backslash_count = 0;
//                     },

//                     _ => {
//                         backslash_count = 0;
//                     }

//                 }
//                 res.push(ch);
    
//             }
//         self.err_handler.unerminated_str_err(self.line + 1, self.column, res);
//     }

//     pub fn parse_code(&mut self, code: &str, is_inner: bool) -> Vec<Token> {
//         let mut tokens = Vec::new();
//         let lines = consts::prepare_code(code);
//         let mut inner_line: usize = 0;
//         let mut inner_col: usize = 0;

//         while inner_line < lines.len() {
//             let line = &lines[inner_line];
//             let rem = if inner_col <= line.0.len() {
//                 &line.0[inner_col..]
//             } else {
//                 ""
//             };

//             if let Some(pos) = rem.find('"') {

//     if pos == 0 {

//         let after_quote = &rem[1..];

//         let (string, rest) =
//             self.read_string(after_quote);

//         tokens.push(
//             self.gen_token(
//                 TokenType::STR(string)
//             )
//         );

//         let consumed =
//             after_quote.len()
//                 .saturating_sub(rest.len());

//         let total_len =
//             1 + consumed;

//         inner_col += total_len;
//         self.column += total_len;

//         continue;
//     }
// }

//             self.column = line.1 + inner_col + 1;

//             if rem.trim().is_empty() || rem.trim().starts_with("//") {
//                 if !is_inner {
//                     tokens.push(self.gen_token(TokenType::NEWLINE));
//                 }

//                 if inner_col >= line.0.len() {

//                     inner_line += 1;
//                     self.line += 1;
//                     inner_col = 0;
//                     self.column = 1;

//                     continue;
//                 }
//             }

//             let mut earl_macth: Option<(&Rule, Captures, usize)> = None;

//             for rule in &self.rules {

//                 if let Some(caps) = rule.re.captures(&rem) {
//                     let start_pos = caps.get(0).unwrap().start();

//                     if earl_macth.as_ref().map_or(true, |(_, _, pos)| start_pos < *pos) {
//                         earl_macth = Some((rule, caps, start_pos));
//                     }
//                 }
//             }

//             if let Some((rule, caps, start_pos)) = earl_macth {
//                 if start_pos > 0 {
//                     self.err_handler.syntax_err(inner_line, inner_col);
//                 }

//                 let matched_str = caps.get(0).unwrap().as_str();
//                 let matched_len = matched_str.len();

//                 if matched_len == 0 {
//                     self.err_handler.internal_err(
//                     self.line,
//                     self.column,
//                     "Zero-length token match".to_string(),
//                     ErrType::Lexer
//                     );
//                 }

//                 println!("matched rule {} line {} col {}", rule.name, self.line, self.column);

//                 match rule.name {

//                     consts::FUNC_DEF => {

//                         let name = self.parse_code(&caps["name"], true);
//                         let args = self.parse_code(&caps["args"], true);

//                         tokens.push(self.gen_token(TokenType::FUNC_DEF));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(name);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(args);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));

//                     },

//                     consts::FUNC_CALL => {

//                         let name = self.parse_code(&caps["name"], true);
//                         let args = self.parse_code(&caps["args"], true);

//                         tokens.push(self.gen_token(TokenType::FUNC_CALL));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(name);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(args);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));

//                     },

//                     consts::RETURN => {

//                         let val = self.parse_code(&caps["val"], true);

//                         tokens.push(self.gen_token(TokenType::RETURN));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(val);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));

//                     },

//                     consts::PRINT => {

//                         let vals = self.parse_code(&caps["val"], true);

//                         tokens.push(self.gen_token(TokenType::PRINT));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(vals);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));

//                     },

//                     consts::ASSIGN => {

//                         let var = self.parse_code(&caps["var"], true);
//                         let val = self.parse_code(&caps["val"], true); 

//                         tokens.push(self.gen_token(TokenType::ASSIGN));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(var);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                         tokens.extend(val);
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));

//                     },

//                     consts::PLUS => {

//                         tokens.push(self.gen_token(TokenType::PLUS));

//                     },

//                     consts::MINUS => {

//                         tokens.push(self.gen_token(TokenType::MINUS));

//                     },

//                     consts::MULTIPLY => {

//                         tokens.push(self.gen_token(TokenType::MULTIPLY));

//                     },

//                     consts::DIVIDE => {

//                         tokens.push(self.gen_token(TokenType::DIVIDE));

//                     },

//                     consts::EXPONENTIATION => {

//                         tokens.push(self.gen_token(TokenType::EXPONENTIATION));

//                     },

//                     consts::DIV_WITH_REMAIN => {

//                         tokens.push(self.gen_token(TokenType::DIV_WITH_REMAIN));

//                     },

//                     consts::REMAIN_FROM_DIV => {

//                         tokens.push(self.gen_token(TokenType::REMAIN_FROM_DIV));

//                     },

//                     consts::FLOAT => {

//                         let val = &caps["val"];

//                         match val.parse::<f64>() {

//                             Ok(v) => {
//                                 if v.is_finite() {
//                                     self.err_handler.number_overflow_err(self.line, self.column, val.to_string());
//                                 } else if v.is_nan() {
//                                     self.err_handler.invalid_value_err(self.line, self.column, val.to_string());
//                                 } else {
//                                     tokens.push(self.gen_token(TokenType::FLOAT(v)));
//                                 }
//                             },
//                             Err(_) => {
                                
//                                 self.err_handler.invalid_value_err(self.line, self.column, val.to_string());

//                             }
                            
//                         }
//                     }

//                     consts::INT => {
                        
//                         let val = &caps["val"];

//                         match val.parse::<i64>() {
//                             Ok(v) => tokens.push(self.gen_token(TokenType::INT(v))),
//                             Err(e) => {

//                                 match e.kind() {

//                                     IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => self.err_handler.number_overflow_err(self.line, self.column, val.to_string()),
//                                     _ => self.err_handler.invalid_value_err(self.line, self.column, val.to_string())
                                    
//                                 }

//                             }
//                         }

//                     }

//                     consts::BOOL => {

//                         let val = &caps["val"];

//                         let bool_val = match val {
//                             consts::TRUE => true,
//                             consts::FALSE => false,
//                             _ => {
//                                 self.err_handler.internal_err(self.line, self.column, format!("Invalid boolean literal: {}", val), ErrType::Lexer);
//                             }
//                         };

//                         tokens.push(self.gen_token(TokenType::BOOL(bool_val)));

//                     },

//                     consts::BLOCK_START => {
//                         tokens.push(self.gen_token(TokenType::BLOCK_START));
//                     },

//                     consts:: BLOCK_END => {
//                         tokens.push(self.gen_token(TokenType::BLOCK_END));
//                     },

//                     consts::COMMA => {
//                         tokens.push(self.gen_token(TokenType::COMMA));
//                     }

//                     consts::VAR => {

//                         let var = caps["var"].to_string();

//                         if let Some(ch) = var.chars().next() {
//                             if ch.is_ascii_digit() || ch == '_' {
//                                 self.err_handler.variable_name_error(self.line, self.column, var);
//                             }
//                         }

//                         tokens.push(self.gen_token(TokenType::VAR(var)));

//                     }

//                     _ => {
//                         self.err_handler.internal_err(self.line, self.column, format!("The lexer found a match for the {} rule but was unable to process it.", rule.name), ErrType::Lexer);
//                     }
//                 }

//                 if matched_len == 0 {
//     self.err_handler.internal_err(
//         self.line,
//         self.column,
//         "Lexer stuck: matched_len == 0".to_string(),
//         ErrType::Lexer
//     );
// }

//                 self.column += matched_len;
//                 inner_col += inner_col.min(line.0.len());
//                 inner_col = inner_col.min(line.0.len());
//             } else {

//                 self.err_handler.syntax_err(self.line, self.column);
            
//             }

//         }

//         return tokens;
//     }
// }