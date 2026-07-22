use crate::{file_manager::span::Span, lexer::structs::TokenType};

#[derive(Debug, Clone)]
pub enum StmtType {
    FuncDef {
        name: String,
        args: Vec<String>,
        body: Vec<StmtType>,
    },
    FuncCall {
        name: String,
        args: Vec<StmtType>,
    },
    Return {
        value: Vec<StmtType>,
    },
    Print {
        value: Vec<StmtType>,
    },
    Input {
        variable: String,
    },

    Add {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    Sub {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    Mul {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    Div {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    Exp {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    DivWithRem {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },
    RemOfDiv {
        a: Box<StmtType>,
        b: Box<StmtType>,
    },

    Var {
        name: String,
    },
    Int {
        value: i64,
    },
    Float {
        value: f64,
    },
    Str {
        value: String,
    },
    Bool {
        value: bool,
    },
}

#[derive(Debug)]
pub enum StmtContent {}

#[derive(Debug)]
pub struct Stmt {
    pub s_type: StmtType,
    pub content: StmtContent,
    pub span_parts: Vec<Span>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RuleItem {
    Token(TokenType),
    Ident,
    Expr,
    List {
        item: Box<RuleItem>,
        sep: TokenType,
        last_sep: Option<TokenType>,
    },
    Optional(Vec<RuleItem>),
    CodeBlock,
}

pub enum RuleMatch {
    NoMatch(usize, RuleItem, TokenType),
    Match(usize),
    CreateAST(Stmt, usize)
}
