use crate::{DebugArgs, consts, errors::ErrHandler, lexer::lexer::Lexer};

pub fn interpret(code: &str, args: Vec<DebugArgs>) {


    let mut lines = Vec::new();
    for l in consts::prepare_code(code) {
        lines.push(l.to_string());
    }
    let err_handler = ErrHandler::new(lines);

    let lexer = Lexer::new();
    let tokens = lexer.parse_code(code, err_handler, false, 1);

    println!("{:?}", tokens);
}