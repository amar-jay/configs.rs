mod options;
use crate::options::{file::FileOptions, Errors, DegreeOfError};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

/// A simple program to organize my CLI commands.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Self explanatory
    command: String,

    /// A regex pattern
    #[arg(short = 'r', required = false)]
    pattern: String,

    #[arg(short, required = false)]
    number: bool,

    /// path of file - regex not supported yet
    #[arg(long, short='p', aliases=["where"], required = true)]
    path: PathBuf,
}

fn run() -> Result<(), Errors> {
    let args = Args::parse();
    let cmd = args.command.as_str();

    match cmd {
        "echo" => FileOptions::echo(args.path),
        "open" => FileOptions::open_file(args.path),
        "cat" => FileOptions::cat_file(args),
        "find" => FileOptions::read_file(args),
        "run" => FileOptions::exec_file(args.path),
        _ => Err(Errors::CommandNotFound),
    }
}

fn print_error(stat: &str, err: Option<Box<dyn std::error::Error>>, deg:DegreeOfError) {
    if let Some(err) = err {
        let err = match deg {
            DegreeOfError::Warn => format!("{}", err).yellow(),
            DegreeOfError::Danger => format!("{}", err).red(),
            DegreeOfError::Info => format!("{}", err).blue(),
        };
     
        eprintln!("{}:\t{}\n", stat.bold().green(), err);
        std::process::exit(1);
    };

    eprintln!("{}:\t{}\n", "Unknown Warning".bold().green(), stat.red());
    std::process::exit(0)
}
fn main() {
    match run() {
        Ok(r) => r,
        Err(Errors::FileNotFound(err)) => print_error("File not found", Some(err), DegreeOfError::Warn),
        Err(Errors::CommandNotFound) => print_error("Command not Found", None, DegreeOfError::Danger),
        Err(Errors::ArgumentError(err)) => print_error("wrong input", Some(err), DegreeOfError::Danger),
       // Err(Errors::FileIsEmpty) => print_error("File not found", None, DegreeOfError::Info),
        Err(Errors::ParsingError(err)) => print_error("Parsing Error", Some(err), DegreeOfError::Info),

        //        Err(Errors::PathError) => eprintln!("Path input error!!"),
    }
}
