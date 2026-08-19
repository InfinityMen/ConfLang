use std::collections::HashMap;

use regex::{Captures, Match};

use crate::{file_manager::file::FileId, lexer::structs::Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub file_id: FileId,

    // all in 0 - based bytes
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Span {
    pub fn char_len(&self, src: &str, start_ofset: usize) -> usize {
        src[start_ofset..start_ofset + self.end_offset - self.start_offset]
            .chars()
            .count()
    }
}

#[derive(Clone)]
pub struct PosIndex {
    pub line_starts: HashMap<FileId, Vec<usize>>,
}

impl PosIndex {
    pub fn new() -> Self {
        Self {
            line_starts: HashMap::new(),
        }
    }

    pub fn push_file(&mut self, code: &str, file_id: FileId) {
        let mut line_starts: Vec<usize> = vec![0];

        for (i, ch) in code.char_indices() {
            if ch == '\n' {
                line_starts.push(i + ch.len_utf8());
            }
        }

        if let Some(_) = self.line_starts.iter().find(|f| f.0 == &file_id) {
            panic!("File #{:?} already exsits in PosIndex", file_id)
        }

        println!("File #{:?}, Line starts: {:?}", file_id, line_starts);

        self.line_starts.insert(file_id, line_starts);
    }

    fn get_line_starts_by_id(&self, id: FileId) -> Vec<usize> {
        if let Some(l) = self.line_starts.iter().find(|x| x.0 == &id) {
            return l.1.clone();
        }

        panic!("File #{:?} not found", id)
    }

    //byte offset  -> line, column
    pub fn byte_to_line_col(&self, file_id: FileId, offset: usize) -> (usize, usize) {
        let line_idx = match self.get_line_starts_by_id(file_id).binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        let line_start = self.get_line_starts_by_id(file_id)[line_idx];
        let col = offset - line_start;

        (line_idx + 1, col)
    }

    pub fn range_to_span(&self, file_id: FileId, start: usize, end: usize) -> Span {
        Span {
            file_id,
            start_offset: start,
            end_offset: end,
        }
    }

    pub fn span_of_match(&self, file_id: FileId, m: Match, cur_pos: usize) -> Span {
        self.range_to_span(file_id, cur_pos, m.end() + cur_pos)
    }

    pub fn span_of_tokens(&self, file_id: FileId, tokens: &[Token]) -> Span {
        let mut start = tokens[0].span.start_offset;
        let mut end = tokens[tokens.len()].span.end_offset;

        return Span {
            file_id,
            start_offset: start,
            end_offset: end,
        };
    }
}
