use std::process::Command;
use clap::Parser;
use std::path::PathBuf;

/// A simple program to organize my CLI commands.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
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


enum Errors {
    CommandNotFound,
    FileNotFound,
    FileIsEmpty,
}

fn read_file(args:Args) -> Result<(), Errors> {

    //let folder = std::fs::read_dir(&args.path).expect("could not read file");
    let content = std::fs::read_to_string(&args.path).map_err(|_| return Errors::FileNotFound)?;

    if content.contains(&args.pattern) {
        return Err(Errors::FileIsEmpty);
    }

    for line in content.lines() {
      if line.contains(&args.pattern) {
        println!("{}", line);
      }
    }

    Ok(())
}

fn open_file(path: PathBuf) -> Result<(), Errors> {
    let path = path.to_str().unwrap();

    //let f = File::options().append(true).open(args.path).map_err(|_| Errors::FileNotFound)?;
    let mut _cmd = Command::new("open_command").current_dir("/home/manan").arg(path).spawn().unwrap();
   // .map_err(|_| Errors::FileNotFound)?;

    return Ok(());
}

fn run() ->  Result<(), Errors> {
    let args = Args::parse();
    let cmd = args.command.as_str();

    match cmd {
        "open" => open_file(args.path), 
        "read" => read_file(args),
        _ => Err(Errors::CommandNotFound)
    }
}

fn main() {
    match run() {
        Ok(r) => r,
        Err(Errors::FileNotFound) => eprintln!("File not found"),
        Err(Errors::CommandNotFound) => eprintln!("Command not Found"),
//        Err(Errors::ArgumentError) => eprintln!("wrong input"),
        Err(Errors::FileIsEmpty) => eprintln!("File not found"),
//        Err(Errors::PathError) => eprintln!("Path input error!!"),
    }
}
