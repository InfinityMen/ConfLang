

mod consts;
pub mod enums;

use core::fmt;
use std::{self, collections::{HashMap, HashSet, hash_map}, fmt::format};

use crate::{consts::Colors, errors::enums::{ErrLevel, ErrPlace, ErrStage}, file_manager::file::FileId, parser, span::{PosIndex, Span}};
use enums::ErrName;


#[derive(Debug, Clone)]
pub struct Error {

    place: ErrPlace,
    name: ErrName,
    level: ErrLevel,
    stage: ErrStage,
    span: Span,
    descr: String

}


pub struct ErrHandler {

    files_ids: HashSet<FileId>,
    console_input: Vec<String>

}


impl ErrHandler {

    pub fn new() -> Self {

        Self { files_ids: HashSet::new(), console_input: Vec::new() }
        
    }


    pub fn push_console_input(&mut self, console_input: Vec<String>) {
        self.console_input = console_input;
    }


    pub fn push_file(&mut self, id: FileId) {

        if !self.files_ids.contains(&id) {
            self.files_ids.insert(id);
        }
        
    }


    pub fn report_err(&self, err: Error) {


        
    }


    pub fn report_warn(&self, err: Error) {

        
        
    }


    pub fn report_debug(&self, err: Error) {

        
        
    }
    

}