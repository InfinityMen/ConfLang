use super::file::{File, FileId};
use crate::consts;
use std::{collections::HashMap, fs, path::{Path, PathBuf}};

#[derive(Debug, Clone)]
pub struct SourceManager {
    files: HashMap<FileId, File>,
    max_id: Option<usize>,
}

impl SourceManager {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            max_id: None,
        }
    }

    pub fn push_file(&mut self, path: &Path) -> FileId {
        let f_path: PathBuf = match fs::canonicalize(&path) {
            Ok(v) => v,
            Err(_) => panic!("Couldn`t cannonicalize file"),
        };

        // check file validity
        if !f_path.is_file() {
            panic!("Something on path {:?} is not file", f_path)
        }

        // checking file extension
        if let Some(e) = f_path.extension().and_then(|ext| ext.to_str()) {
            if !consts::ALLOWED_EXTENSIONS.contains(&e) {
                panic!("Incorrect file extension")
            }
        }

        if let Some(f) = self.files.iter().find(|&i| i.1.absolute_path == f_path) {
            return f.0.clone();
        }

        // reading file
        let text = self.read_file(f_path.clone());

        // setting file ID
        let file_id: FileId;

        if let Some(m) = self.max_id {
            self.max_id = Some(m + 1);
            file_id = FileId(m + 1);
        } else {
            self.max_id = Some(0);
            file_id = FileId(0)
        };

        self.files.insert(
            file_id,
            File {
                absolute_path: f_path,
                code: text.into(),
            },
        );

        file_id
    }

    fn read_file(&self, path: PathBuf) -> String {
        let txt = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => panic!("file {:?} not found", path),
        };

        txt
    }

    pub fn get_file_path(&self, id: FileId) -> String {
        if let Some(f) = self.files.get(&id) {
            return f.absolute_path.to_string_lossy().into_owned();
        }

        panic!("file #{:?} does not exists", id)
    }

    pub fn get_file_src(&self, id: FileId) -> &str {
        if let Some(f) = self.files.get(&id) {
            return &f.code;
        }

        panic!("file #{:?} does not exists", id)
    }
}
