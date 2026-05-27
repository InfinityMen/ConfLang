use std::collections::HashMap;

use regex::{Captures, Match};

use crate::file_manager::file::FileId;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {

    pub file_id: FileId,

    // all in bytes

    pub start_line: usize, // 1 - based
    pub start_col: usize, // 1 - based
    pub start_offset: usize, // 0 - based

    pub end_line: usize, // 1 - based
    pub end_col: usize, // 1 - based
    pub end_offset: usize, // 0 - based

    pub byte_len: usize,

}

impl Span {

    pub fn char_len(&self, src: &str, start_ofset: usize) -> usize {

        src[start_ofset..start_ofset + self.byte_len].chars().count()
        
    }
    
}


#[derive(Clone)]
pub struct PosIndex {

    pub line_starts: HashMap<FileId, Vec<usize>>

}

impl PosIndex {

    pub fn new() -> Self {

        Self { line_starts: HashMap::new() }
        
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

        let (start_line, start_col) = self.byte_to_line_col(file_id, start);

        let (end_line, end_col) = self.byte_to_line_col(file_id, end);

        Span {

            file_id,
            start_line,
            start_col,
            start_offset: start,
            end_line,
            end_col,
            end_offset: end,
            byte_len: end - start

        }
        
    }

    pub fn span_of(&self, file_id: FileId, m: Match) -> Span {
        self.range_to_span(file_id, m.start(), m.end())
    }

    pub fn span_of_caps(&self, file_id: FileId, caps: Captures) -> Span {
        let m = caps.get(0).expect("caps[0] not found");
        self.span_of(file_id, m)
    }

    pub fn span_of_group(&self, file_id: FileId, caps: Captures, name: &str) -> Option<Span> {
        caps.name(name).map(|m| self.span_of(file_id, m))
    }
    
}