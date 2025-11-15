// main.rs
use std::env;
use std::error::Error;
use std::fs;
use std::io;

const VERSION: &str = "1.0.0";

/// CLI utility to read and print the contents of a file.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Version: {}", VERSION);
            println!("Usage: file_reader <filename>");
        }
        2 => {
            let filename = &args[1];
            read_and_print_file(filename)?;
        }
        _ => {
            eprintln!("Error: Invalid number of arguments.");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Reads and prints the contents of a file.
fn read_and_print_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    println!("{}", contents);

    Ok(())
}

// Custom error type for file I/O errors.
#[derive(Debug)]
struct FileError {
    msg: String,
}

impl FileError {
    fn new(msg: &str) -> Self {
        FileError {
            msg: msg.to_string(),
        }
    }
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// Custom result type for file I/O results.
type FileResult<T> = Result<T, io::Error>;