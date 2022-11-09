use std::{fs::File, io::Read, path::PathBuf, process::Command};

use crate::Args;
use crate::Errors;

pub struct FileOptions {}

impl FileOptions {
    /// This echos the pattern entered
    pub fn echo(path: PathBuf) -> Result<(), Errors> {
        if path.to_str().is_none() {
            return Err(Errors::ArgumentError(Box::new(std::fmt::Error)));
        }
        println!("{}", path.to_str().unwrap());
        Ok(())
    }

    /// read a given file from path
    pub fn read_file(args: Args) -> Result<(), Errors> {
        //let folder = std::fs::read_dir(&args.path).expect("could not read file");
        //let content = std::fs::read_to_string(&args.path).map_err(|_| return Errors::FileNotFound)?;
        let mut f = File::options()
            .append(true)
            .open(args.path)
            .map_err(|err| Errors::FileNotFound(Box::new(err)))?;
        let mut content = String::from("");
        f.read_to_string(&mut content)
            .map_err(|err| Errors::FileNotFound(Box::new(err)))?;

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
