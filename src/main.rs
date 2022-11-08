#![allow(unused)]
use clap::Parser;
use log::{info, warn};
use std::env;
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


struct Error(String);
fn main() ->  Result<(),Box<dyn std::error::Error> > {
    let args = Args::parse();
    env_logger::init();

    println!("path: {:?} \npattern : {}", args.path, args.pattern );
    //let folder = std::fs::read_dir(&args.path).expect("could not read file");
    let content = std::fs::read_to_string(&args.path).map_err(|_| info!("unable to find {:?}", args.path));
    for line in content.lines() {
      if line.contains(&args.pattern) {
        println!("{}", line);
      }
    }

    Ok(())
}
