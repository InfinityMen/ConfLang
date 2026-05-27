use regex::{Captures, Match};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {

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

    pub line_starts: Vec<usize>

}

impl PosIndex {

    pub fn new(code: &str) -> Self {

        let mut line_starts: Vec<usize> = vec![0];

        for (i, ch) in code.char_indices() {

            if ch == '\n' {
                line_starts.push(i + ch.len_utf8());
            }
            
        }

        println!("Line starts: {:?}", line_starts);

        Self { line_starts }
        
    }
                                    //byte offset  -> line, column
    pub fn byte_to_line_col(&self, offset: usize) -> (usize, usize) {

        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        let line_start = self.line_starts[line_idx];
        let col = offset - line_start;

        (line_idx + 1, col)
        
    }

    pub fn range_to_span(&self, start: usize, end: usize) -> Span {

        let (start_line, start_col) = self.byte_to_line_col(start);

        let (end_line, end_col) = self.byte_to_line_col(end);

        Span {

            start_line,
            start_col,
            start_offset: start,
            end_line,
            end_col,
            end_offset: end,
            byte_len: end - start

        }
        
    }

    pub fn span_of(&self, m: Match) -> Span {
        self.range_to_span(m.start(), m.end())
    }

    pub fn span_of_caps(&self, caps: Captures) -> Span {
        let m = caps.get(0).expect("caps[0] not found");
        self.span_of(m)
    }

    pub fn span_of_group(&self, caps: Captures, name: &str) -> Option<Span> {
        caps.name(name).map(|m| self.span_of(m))
    }
    
}