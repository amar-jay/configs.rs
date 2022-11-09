pub mod file;
use std::path::PathBuf;
use crate::Args;

pub enum Errors {
    ArgumentError,
    CommandNotFound,
    FileNotFound,
    FileIsEmpty,
}

impl Args {

    #![allow(unused)]
    fn new(command: String, pattern: String, path: PathBuf) -> Args {
        Args {
            command,
            pattern,
            path
        }
    }
}



