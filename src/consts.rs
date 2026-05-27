use colored::*;



pub const NEWLINE: &str = "NEWLINE";
pub const WHITESPACE: &str = "WHITESPACE";


// func_def
pub const TO: &str = "TO";
pub const STUDY: &str = "STUDY";
pub const THE: &str = "THE";
pub const RITUAL: &str = "RITUAL";
pub const OF: &str = "OF";
pub const WITH: &str = "WITH";
pub const IS: &str = "IS";
pub const A: &str = "A";
pub const PLEASURE: &str = "PLEASURE";


// func_call
pub const LET: &str = "LET";
pub const US: &str = "US";
pub const CONSULT: &str = "CONSULT";
pub const WISDOM: &str = "WISDOM";
pub const REGRADING: &str = "REGRADING";


// return
pub const BEGINNING: &str = "BEGINNING";
pub const DEFINITION: &str = "DEFINITION";
pub const TERMS: &str = "TERMS";


// print
pub const HE: &str = "HE";
pub const FIRST: &str = "FIRST";
pub const PRESENTS: &str = "PRESENTS";
pub const HIS: &str = "HIS";
pub const WORDS: &str = "WORDS";
pub const AS: &str = "AS";
pub const THEN: &str = "THEN";
pub const ACCORDING: &str = "ACCORDING";
pub const THEM: &str = "THEM";
pub const ACTS: &str = "ACTS";


// assign
pub const WHO: &str = "WHO";
pub const BY: &str = "BY";
pub const REANIMATING: &str = "REANIMATING";
pub const CAN: &str = "CAN";
pub const GAIN: &str = "GAIN";
pub const KNOWLEGE: &str = "KNOWLEGE";

// operators (by words)
pub const HARMONIZED: &str = "HARMONIZED";
pub const DIMINISHED: &str = "DIMINISHED";
pub const MULTIPLED: &str = "MULTIPLED";
pub const SHARED: &str = "SHARED";
pub const AMONG: &str = "AMONG";
pub const RAISED: &str = "RAISED";
pub const POWER: &str = "POWER";
pub const PARTITIONED: &str = "PARTITIONED";
pub const REMAINDER: &str = "REMAINDER";


// function word
pub const AND: &str = "AND";


// bool
pub const YANG: &str = "YANG";
pub const YIN: &str = "YIN";


// symbols
pub const COMMA: &str = "СOMMA";
pub const DOT: &str = "DOT";
pub const LBRACE: &str = "LBRACE";
pub const RBRACE: &str = "RBRACE";


// nums
pub const FLOAT: &str = "FLOAT";
pub const INT: &str = "INT";


// string
pub const QUOTE: &str = "QUOTE";
pub const STR: &str = "I_NEVER_MATCHES";


// vars
pub const VAR: &str = "VAR";


// module for easy use of common colors
const FIRM_COLOR: (u8, u8, u8) = (0, 162, 255); // #00a2ff
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

pub fn prepare_code<'a> (code: &str) -> Vec<&str> {

    code.lines().collect()

    }

pub trait DebugCheck {
    fn check_debug(&self, debug: bool) -> &str;
}

impl DebugCheck for &str {
    fn check_debug(&self, debug: bool) -> &str {
        if debug {
            self
        } else {
            ""
        }
    }
}

pub const ALLOWED_EXTENSIONS: [&str; 2] = ["clg", "txt"];