use std::fs::File;
use std::{io, path};

enum MyError {
    NotFound,
    Io(io::Error),
}

fn OpenFile(path: &str ) -> Result<String, MyError> {
    let file = File::open(path).map_err(MyError::Io)?;
    Ok("FIle opened Successfully !".to_string())
}

fn main() {
    match OpenFile("Missing.txt") {
        Ok(msg) => println!("{}",msg),
        Err(MyError::NotFound) => println!("File not found"),
        Err(MyError::Io(e)) => println!("Io Error {}",e),
    }
}