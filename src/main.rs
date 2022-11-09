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
    #[arg(short='n', required=false)]
    pattern: String,

    /// path of file - regex not supported yet
    #[arg(long, short='p', aliases=["where"], required=true)]
    path: PathBuf,
}

fn run() ->  Result<(), Errors> {
    let args = Args::parse();
    let cmd = args.command.as_str();

    match cmd {
        "echo" => FileOptions::echo(args.path),
        "open" => FileOptions::open_file(args.path), 
        "read" => FileOptions::read_file(args),
        "run"  => FileOptions::exec_file(args.path),
            _  => Err(Errors::CommandNotFound)
    }
}

fn print_error(stat: &str) {
    eprintln!("{}",stat);
    std::process::exit(1);
}
fn main() {
    match run() {
        Ok(r) => r,
        Err(Errors::FileNotFound) => print_error("File not found"),
        Err(Errors::CommandNotFound) => print_error("Command not Found"),
        Err(Errors::ArgumentError) => print_error("wrong input"),
        Err(Errors::FileIsEmpty) => print_error("File not found"),
//        Err(Errors::PathError) => eprintln!("Path input error!!"),
    }
}
