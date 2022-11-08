#![allow(unused)]
use clap::Parser;
use log::{info, warn};
use std::{env, error};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,
    path: PathBuf,
}

impl Args {
    fn new(pattern: String, path: PathBuf) -> Args {
        Args {
            pattern,
            path
        }
    }
}


#[derive(Debug)]
enum Errors {
    FileNotFound,
    ArgumentError,
    PathError
}
fn run() ->  Result<(), Errors> {
    let args = Args::parse();
    //env_logger::init();

    println!("path: {:?} \npattern : {}", args.path, args.pattern );
    //let folder = std::fs::read_dir(&args.path).expect("could not read file");
    let content = std::fs::read_to_string(&args.path).unwrap_or(|_| return Errors::FileNotFound);

    for line in content.lines() {
      if line.contains(&args.pattern) {
        println!("{}", line);
      }
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(r) => r,
        Errors::FileNotFound => info!("File not found");
        Errors::ArgumentError => warn!("wrong input");
        Errors::PathError => warn!("Path input error!!");
        _ => ();,
    }
}
