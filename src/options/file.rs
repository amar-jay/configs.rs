use std::{process::Command, fs::File, path::PathBuf, io::Read};
use crate::Errors;
use crate::Args;
pub struct FileOptions {}

impl FileOptions {
    /// This echos the pattern entered
    pub fn echo(path: PathBuf) -> Result<(), Errors> {
        if path.to_str().is_none() {
            return Err(Errors::ArgumentError);
        }
        println!("{}", path.to_str().unwrap());
        Ok(())
    }

    /// read a given file from path
    pub fn read_file(args:Args) -> Result<(), Errors> {

        //let folder = std::fs::read_dir(&args.path).expect("could not read file");
        //let content = std::fs::read_to_string(&args.path).map_err(|_| return Errors::FileNotFound)?;
        let mut f = File::options().append(true).open(args.path).map_err(|_| Errors::FileNotFound)?;
        let mut content = String::from("");
        f.read_to_string(&mut content).map_err(|_| Errors::FileIsEmpty)?;

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

    pub fn exec_file(path: PathBuf) -> Result<(), Errors> {
        let path = path.to_str().unwrap();

        Command::new(path).spawn().map_err(|_| Errors::ArgumentError)?;

        return Ok(());
    }

    pub fn open_file(path: PathBuf) -> Result<(), Errors> {
        let path = path.to_str().unwrap();

        Command::new("open_command").current_dir("/home/manan").arg(path).spawn().unwrap();
       // .map_err(|_| Errors::FileNotFound)?;

        return Ok(());
    }

}


