use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub struct File {
    // pub id: FileId,
    // pub name: String, // logical name, like "utils.lexer" or "main"
    pub absolute_path: PathBuf,
    pub code: Arc<str>, // content of file
                        // PosIndex in special module
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub usize);
