use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Token {

    pub token_type: TokenType,

    pub span: Span

}


#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

    NEWLINE,
    WHITESPACE,

    // func_def
    TO,
    STUDY,
    THE,
    RITUAL,
    OF,
    WITH,
    IS,
    A,
    PLEASURE,

    // func_call
    LET,
    US,
    CONSULT,
    WISDOM,
    REGRADING,

    // return
    BEGINNING,
    DEFINITION,
    TERMS,

    // print
    HE,
    FIRST,
    PRESENTS,
    HIS,
    WORDS,
    AS,
    THEN,
    ACCORDING,
    THEM,
    ACTS,

    // assign
    WHO,
    BY,
    REANIMATING,
    CAN,
    GAIN,
    KNOWLEGE,

    // operators (by words)
    HARMONIZED,
    DIMINISHED,
    MULTIPLED,
    SHARED,
    AMONG,
    RAISED,
    POWER,
    PARTITIONED,
    REMAINDER,

    // function words
    AND,

    // chars
    COMMA,
    DOT,
    LBRACE,
    RBRACE,
    
    // data
    VAR(String),
    INT(i64),
    FLOAT(f64),
    STR(String),
    BOOL(bool),
}
