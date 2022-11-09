pub mod file;
use std::path::PathBuf;
use crate::Args;

type BoxedError = Box<dyn std::error::Error>;
pub enum Errors {
    ArgumentError(BoxedError),
    CommandNotFound,
    FileNotFound(BoxedError),
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



