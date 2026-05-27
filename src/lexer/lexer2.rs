// use std::vec;
// use crate::errors::ErrHandler;
// use regex::{Captures, Regex,};
// use super::structs::{Token, TokenType};

// use crate::consts;


// pub struct Rule {
//     name: &'static str,
//     re: Regex,
// }

// pub struct Lexer {
//     rules: Vec<Rule>,
// }

// impl Lexer {
//     pub fn new() -> Self {
//         Self {
//             rules: vec![
//                 Rule { 
//                     name: consts::FUNC_DEF, 
//                     re: Regex::new(r"^To study the ritual of \[\s*(?P<name>\w+)\s*\] with \[\s*(?P<args>.*)\s*\] is a pleasure\.").unwrap() 
//                 },
//                 Rule { 
//                     name: consts::FUNC_VOID, 
//                     re: Regex::new(r"^The gentleman applies himself to the root, for \[\s*(?P<name>\w+)\s*\] and its \[\s*(?P<args>.*)\s*\] shall be made manifest\.").unwrap() 
//                 },
//                 Rule {
//                     name: consts::FUNC_CALL, 
//                     re: Regex::new(r"^Let us consult the wisdom of \[\s*(?P<name>\w+)\s*\] regarding \[\s*(?P<args>.*)\s*\]\.").unwrap()
//                 },
//                 Rule {
//                     name: consts::RETURN,
//                     re: Regex::new(r"^The beginning of wisdom is the definition of terms \[\s*(?P<val>.*)\s*\]\.").unwrap(),
//                 },
//                 Rule { 
//                     name: consts::PRINT, 
//                     re: Regex::new(r"^He first presents his words as \[\s*(?P<val>.*)\s*\], and then according to them he acts\.").unwrap() 
//                 },
//                 Rule { 
//                     name: consts::ASSIGN, 
//                     re: Regex::new(r"^He who by reanimating \[\s*(?P<val>.*)\s*\] can gain knowledge of \[\s*(?P<var>.*)\s*\]\.").unwrap() 
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
//                     re: Regex::new(r#"^\s*(?P<val>-?\d+\.\d+)(?:\s|,|\]|$)"#).unwrap()},
//                 Rule {
//                     name: consts::INT, 
//                     re: Regex::new(r"^\s*(?P<val>-?\d+)(?:\s|,|\]|$)").unwrap() },
//                 Rule {
//                     name: consts::STR, 
//                     re: Regex::new(r#"^\s*"(?P<val>(?:[^"\\]|\\.)*)""#).unwrap()},
//                 Rule {
//                     name: consts::UNTERMINATED_STR,
//                     re: Regex::new(r#"^\s*"(?P<val>(?:[^"\\]|\\.)*)"#).unwrap()},
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
//                     re: Regex::new(r"^\s*(?P<var>[a-zA-Z_]\w*)").unwrap()
//             },
//             ],
//         }
//     }


//     fn gen_token(&self, token: TokenType, line_index: usize, column: usize) -> Token {
//         Token { token_type: token, line: line_index + 1, column: column }
//     }
    

//     pub fn parse_code(&self, code: &str, err_handler: &mut ErrHandler, is_inner: bool, main_line: usize) -> Vec<Token> {
//         let mut tokens:Vec<Token> = Vec::new();
//         let lines = consts::prepare_code(code);
//         let mut line_index=0;
//         let mut cur_pos_in_line:usize = 0;
//         let mut main_line = main_line;

//         while line_index < lines.len() {
//             let line = &lines[line_index];
//             let remaining = &line[cur_pos_in_line..];

//             if remaining.is_empty() || remaining.starts_with("//") {
//                 if !is_inner {
//                     tokens.push(self.gen_token(TokenType::NEWLINE, main_line, cur_pos_in_line));
//                 }
//                 line_index += 1;
//                 main_line += 1;
//                 cur_pos_in_line = 0;
//                 continue;
//             };

            

//             let mut earl_macth: Option<(&Rule, Captures, usize)> = None;

//             for rule in &self.rules {
//                 if let Some(caps) = rule.re.captures(&remaining) {
//                     let start_pos = caps.get(0).unwrap().start();

//                     if earl_macth.as_ref().map_or(true, |(_, _, pos)| start_pos < *pos) {
//                         earl_macth = Some((rule, caps, start_pos));
//                     }
//                 }
//             }

//             if let Some((rule, caps, start_pos)) = earl_macth {
//                 if start_pos > 0 {
//                     let ch = remaining
//                         .chars()
//                         .next()
//                         .unwrap();

//                     err_handler.unused_char_err(main_line, cur_pos_in_line, ch);

//                     cur_pos_in_line += ch.len_utf8();
//                     continue;

//                 }

//                 let matched_str = caps.get(0).unwrap().as_str();
//                 let matched_len = matched_str.len();

//                 match rule.name {
                
//                 consts::FUNC_DEF => {
//                     let name = &caps["name"];
//                     let args = self.parse_code(&caps["args"], err_handler, true, main_line);
                    
//                     tokens.push(self.gen_token(TokenType::FUNC_DEF(name.to_string()), main_line, cur_pos_in_line));

//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(args);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));

                    
//                 },

//                 consts::FUNC_VOID => {
//                     let name = &caps["name"];
//                     let args = self.parse_code(&caps["args"], err_handler, true, main_line);

//                     tokens.push(self.gen_token(TokenType::FUNC_VOID(name.to_string()), main_line, cur_pos_in_line));
//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(args);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));
                    
                    
//                 },

//                 consts::FUNC_CALL => {
//                     let name = &caps["name"];
//                     let args = self.parse_code(&caps["args"], err_handler, true, main_line);

//                     tokens.push(self.gen_token(TokenType::FUNC_CALL(name.to_string()), main_line, cur_pos_in_line));
//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(args);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));

                    
                    
//                 },

//                 consts::RETURN => {
//                     tokens.push(self.gen_token(TokenType::RETURN, main_line, cur_pos_in_line));
//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(self.parse_code(&caps["val"].to_string(), err_handler, true, main_line));
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::PRINT => {
//                     let vals = self.parse_code(&caps["val"], err_handler, true, main_line);

//                     tokens.push(self.gen_token(TokenType::PRINT, main_line, cur_pos_in_line));

//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(vals);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));

                    
//                 },

//                 consts::ASSIGN => {
//                     let val = self.parse_code(&caps["val"], err_handler, true, main_line);
//                     let var = self.parse_code(&caps["var"], err_handler, true, main_line);

//                     tokens.push(self.gen_token(TokenType::ASSIGN, main_line, cur_pos_in_line));
//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(var);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));

//                     tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
//                     tokens.extend(val);
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));

                    
//                 },

//                 consts::PLUS => {
//                     tokens.push(self.gen_token(TokenType::PLUS, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::MINUS => {
//                     tokens.push(self.gen_token(TokenType::MINUS, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::MULTIPLY => {
//                     tokens.push(self.gen_token(TokenType::MULTIPLY, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::DIVIDE => {
//                     tokens.push(self.gen_token(TokenType::DIVIDE, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::EXPONENTIATION => {
//                     tokens.push(self.gen_token(TokenType::EXPONENTIATION, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::DIV_WITH_REMAIN => {
//                     tokens.push(self.gen_token(TokenType::DIV_WITH_REMAIN, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::REMAIN_FROM_DIV => {
//                     tokens.push(self.gen_token(TokenType::REMAIN_FROM_DIV, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::FLOAT => {
//                     let val = &caps["val"];

//                     match val.parse::<f64>() {
//                         Ok(v) => {
//                             tokens.push(self.gen_token(TokenType::FLOAT(v), main_line, cur_pos_in_line));
//                         },

//                         Err(_) => {
//                             err_handler.invalid_value_err(main_line,cur_pos_in_line,val.to_string());
//                         }
//                     };
                    
//                 },

//                 consts::INT => {
//                     let val = &caps["val"];

//                     match val.parse::<i64>() {
//                         Ok(v) => {
//                             tokens.push(self.gen_token(TokenType::INT(v), main_line, cur_pos_in_line));
//                         },

//                         Err(_) => {
//                             err_handler.number_overflow_err(main_line, cur_pos_in_line, val.to_string());
//                         }
//                     };
                    
//                 },

//                 consts::STR => {
//                     let val = &caps["val"];

//                     if !val.trim_end().ends_with("\"") {
                        
//                     }

//                     tokens.push(self.gen_token(TokenType::STR(val.to_string()), main_line, cur_pos_in_line));
//                 }

//                 consts::UNTERMINATED_STR => {
//                     let val = &caps["val"];

//                     err_handler.unerminated_str_err( main_line, cur_pos_in_line, val.to_string());

//                     return tokens;
//                 }

//                 consts::BOOL => {
//                     let val = &caps["val"] == consts::TRUE;
//                     tokens.push(self.gen_token(TokenType::BOOL(val), main_line, cur_pos_in_line));
//                 }

//                 consts::BLOCK_START => {
//                         tokens.push(self.gen_token(TokenType::BLOCK_START, main_line, cur_pos_in_line));
                        
//                 },

//                 consts::BLOCK_END => {
//                     tokens.push(self.gen_token(TokenType::BLOCK_END, main_line, cur_pos_in_line));
                    
//                 },

//                 consts::COMMA => {
//                     tokens.push(self.gen_token(TokenType::COMMA, main_line, cur_pos_in_line));
//                 },

//                 consts::VAR => {
//                 let var = &caps["var"];

//                     tokens.push(self.gen_token(TokenType::VAR(var.to_string()), main_line, cur_pos_in_line));
//                 }
                
//                 _ => {
//                     err_handler.internal_err(line_index, cur_pos_in_line, "Found unknown token".to_string());
//                 }
//                 }
//                 cur_pos_in_line += matched_len;
//             } else {
//                 err_handler.syntax_err(
//         main_line,
//         cur_pos_in_line,);

//     let ch =
//         remaining
//         .chars()
//         .next()
//         .unwrap();

//     cur_pos_in_line
//         += ch.len_utf8();
//             }
//         }

//             return tokens;
//         }
//         }
