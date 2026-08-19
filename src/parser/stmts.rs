use crate::{file_manager::span::Span, lexer::structs::TokenType};

#[derive(Debug, Clone)]
pub enum StmtType {
    FuncDef,
    FuncCall,
    Return,
    Print,
    // Input,

    Add,
    Sub,
    Mul,
    Div,
    Exp,
    DivWithRem,
    RemOfDiv,

    Var,
    Int,
    Float,
    Str,
    Bool
}

#[derive(Debug)]
pub enum StmtContent {
    FuncDef {
        name: Box<Stmt>,
        args: Vec<Stmt>,
        body: Vec<Stmt>,
    },
    FuncCall {
        name: Box<Stmt>,
        args: Vec<Stmt>,
    },
    Return {
        value: Vec<Stmt>,
    },
    Print {
        value: Vec<Stmt>,
    },
    // Input {
    //     variable: String,
    // },

    Add {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    Sub {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    Mul {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    Div {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    Exp {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    DivWithRem {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },
    RemOfDiv {
        a: Box<Stmt>,
        b: Box<Stmt>,
    },

    Var {
        name: Vec<String>,
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
pub struct Stmt {
    pub s_type: StmtType,
    pub content: StmtContent,
    pub span: Span,
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
