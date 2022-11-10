pub mod file;
//use crate::Args;
//use std::path::PathBuf;

type BoxedError = Box<dyn std::error::Error>;
pub enum Errors {
    ArgumentError(BoxedError),
    CommandNotFound,
    FileNotFound(BoxedError),
    ParsingError(BoxedError),
 //   FileIsEmpty,
}

pub enum DegreeOfError {
    Warn,
    Danger,
    Info,
}
/*
impl Args {
    #![allow(unused)]
    fn new(command: String, pattern: String, path: PathBuf) -> Args {
        Args {
            command,
            number: false,
            pattern,
            path,
        }
    }
}
*/
