use crate::file_manager::{file::FileId, span::Span};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    name: ErrName,
    level: ErrLevel,
    stage: ErrStage,
    span: Span,
    descr: String,
}

#[derive(Debug, Clone)]
pub enum ErrStage {
    Console,
    Lexer,
    Parser,
    Runtime,
}

#[derive(Debug, Clone)]
pub enum ErrName {
    InvalidConsoleArgument,
    InvalidExtension,
    ModuleNotFound,
    

    IncorrectSyntax,
    UnusedCharacter,
    UnterminatedString,

    InvalidValue,
    NumberOverflow,

    MisssingBracket,
    InvalidExpression,

    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    VariableNameError,
    VariableWarn,

    DivisionByZero,

    InternalError,

    UnusedVar,
    UnusedBracets,
}

impl fmt::Display for ErrName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrName::InvalidConsoleArgument => write!(f, "InvalidConsoleArgumentError"),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrLevel {
    Error,
    Warning,
    Debug,
}

impl fmt::Display for ErrLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrLevel::Error => write!(f, "error"),
            ErrLevel::Warning => write!(f, "warning"),
            ErrLevel::Debug => write!(f, "debug"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrPlace {
    Console,
    File(FileId),
}
