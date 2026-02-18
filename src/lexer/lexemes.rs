pub struct LexTemplate {
    pub id: u32,
    pub name: &'static str,
    pub pattern: &'static str,
}

pub const LEXEMES: &[LexTemplate] = &[
    LexTemplate {
        id: 10,
        name: "BLOCK_START",
        pattern: r"\{"
    },

    LexTemplate {
        id: 11,
        name: "BLOCK_END",
        pattern: r"\}"
    },

    LexTemplate {
        id: 20,
        name: "ASSIGN",
        pattern: r"He who by reanimating \[(?P<expr>.*)\] can gain knowledge of \[(?P<var>.*)\]\."
    },

    LexTemplate {
        id: 21,
        name: "PRINT",
        pattern: r"He first presents his words as \[(?P<expr>)\], and then according to them he acts."
    },
];