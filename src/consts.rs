use colored::*;

pub const BLOCK_START: &str = "BLOCK_START";
pub const BLOCK_END: &str = "BLOCK_END";
pub const COMMA: &str = "COMMA";
pub const PLUS: &str = "PLUS";
pub const MINUS: &str = "MINUS";
pub const MULTIPLY: &str = "MULTIPLY";
pub const DIVIDE: &str = "DIVIDE";
pub const EXPONENTIATION: &str = "EXPONENTIATION";
pub const DIV_WITH_REMAIN: &str = "DIV_WITH_REMAIN";
pub const REMAIN_FROM_DIV: &str = "REMAIN_FROM_DIV";
pub const ASSIGN: &str = "ASSIGN";
pub const FUNC_DEF: &str = "FUNC_DEF";
pub const FUNC_VOID: &str = "FUNC_VOID";
pub const FUNC_CALL: &str = "FUNC_CALL";
pub const RETURN: &str = "RETURN";
pub const PRINT: &str = "PRINT";
pub const INPUT: &str = "INPUT";
pub const FLOAT: &str = "FLOAT";
pub const INT: &str = "INT";
pub const BOOL: &str = "BOOL";
pub const STR: &str = "STR";
pub const VAR: &str = "VAR";
pub const TRUE: &str = "Yang";

pub const SYNTAX_ERROR : &str = "SyntaxError";
pub const VALUE_OVERFLOW_ERROR : &str = "ValueOverflowError";
pub const ZERO_DIVISION_ERROR : &str = "ZeroDivisionError";
pub const NAME_EREROR : &str = "NameError";
pub const TOKEN_NOT_FOUND_ERR : &str = "TokenNotFoundErr";


const FIRM_COLOR: (u8, u8, u8) = (0, 162, 255); // #00A2FF
const GREY: (u8, u8, u8) = (89, 89, 89);
pub trait Colors {
    fn err(&self) -> ColoredString;
    fn firm_color(&self) -> ColoredString;
    fn grey(&self) -> ColoredString;
}

impl Colors for &str {
    fn err(&self) -> ColoredString {
        self.red().bold()
    }
    fn firm_color(&self) -> ColoredString {
        self.truecolor(FIRM_COLOR.0, FIRM_COLOR.1, FIRM_COLOR.2)
    }
    fn grey(&self) -> ColoredString {
        self.truecolor(GREY.0, GREY.1, GREY.2)
    }
}