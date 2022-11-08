mod options;
use crate::options::{file::FileOptions, Errors};
use clap::Parser;
use std::path::PathBuf;

/// A simple program to organize my CLI commands.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Self explanatory
    command: String,
    /// A regex pattern
    #[arg(long, short='r', required=false)]
    pattern: String,

    /// path of file - regex not supported yet
    #[arg(long, short='p', aliases=["where"], required=true)]
    path: PathBuf,
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

fn run() ->  Result<(), Errors> {
    let args = Args::parse();
    let cmd = args.command.as_str();

    match cmd {
        "open" => FileOptions::open_file(args.path), 
        "read" => FileOptions::read_file(args),
        "run"  => FileOptions::exec_file(args.path),
            _  => Err(Errors::CommandNotFound)
    }
}

fn main() {
    match run() {
        Ok(r) => r,
        Err(Errors::FileNotFound) => eprintln!("File not found"),
        Err(Errors::CommandNotFound) => eprintln!("Command not Found"),
        Err(Errors::ArgumentError) => eprintln!("wrong input"),
        Err(Errors::FileIsEmpty) => eprintln!("File not found"),
//        Err(Errors::PathError) => eprintln!("Path input error!!"),
    }
}
