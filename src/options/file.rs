use std::{fs::File, io::{BufReader, BufRead}, path::PathBuf, process::Command};

use crate::Args;
use colored::Colorize;
use crate::Errors;

pub struct FileOptions {}

impl FileOptions {
    /// This echos the pattern entered
    fn open(name: &str) -> Result<Box<dyn BufRead>, Errors> {
        let f = File::open(name)
            .map_err(|err| Errors::FileNotFound(Box::new(err)))?;

        let content = BufReader::new(f);
        Ok(Box::new(content))
    }
    pub fn echo(path: PathBuf) -> Result<(), Errors> {
        if path.to_str().is_none() {
            return Err(Errors::ArgumentError(Box::new(std::fmt::Error)));
        }
        println!("{}", path.to_str().unwrap().blue());
        Ok(())
    }

    /// Similar to cat on linux
    pub fn cat_file(args: Args) -> Result<(), Errors> {
        let content = Self::open(args.path.to_str().unwrap())?;

        for (line_no, line) in content.lines().enumerate() {
            let line = line.map_err(|err| Errors::ParsingError(Box::new(err)))?;

                if args.number {
                println!("{}:\t{}", line_no.to_string().blue(), line);
                } else {
                    println!("\t{}", line);
                }
        }
        Ok(())
    }
    /// read a given file from path
    pub fn read_file(args: Args) -> Result<(), Errors> {
        //let folder = std::fs::read_dir(&args.path).expect("could not read file");
        //let content = std::fs::read_to_string(&args.path).map_err(|_| return Errors::FileNotFound)?;
        // k
       // let mut content = String::from("");
       // f.read_to_string(&mut content)
       //     .map_err(|err| Errors::ParsingError(Box::new(err)))?;
        let content = Self::open(args.path.to_str().unwrap())?;

        print!("\n");
        for (line_no, line) in content.lines().enumerate() {
            let line = line.map_err(|err| Errors::ParsingError(Box::new(err)))?;
            if line.contains(&args.pattern) {
                let line:Vec<_> = line.split_whitespace().into_iter().map(|e| {
                    if e == args.pattern {
                        return e.to_string().green().to_string()
                    }
                    e.to_string()
                }).collect();
                if args.number {
                println!("{}:\t{}", line_no.to_string().blue(), line.join(" "));
                } else {
                    println!("\t{}", line.join(" "));
                }
            }
        }

        Ok(())
    }

    /// execute a given file from path
    pub fn exec_file(path: PathBuf) -> Result<(), Errors> {
        let path = path.to_str().unwrap();
        Command::new(path)
            .spawn()
            .map_err(|err| Errors::ArgumentError(Box::new(err)))?;
        Ok(())
    }

    /// open a given file from path
    pub fn open_file(path: PathBuf) -> Result<(), Errors> {
        let path = path.to_str().unwrap();

        // TODO: resolve Errors
        Command::new("open_command")
            .current_dir("/home/manan")
            .arg(path)
            .spawn()
            .map_err(|err| Errors::FileNotFound(Box::new(err)))?;

        Ok(())
    }
}
