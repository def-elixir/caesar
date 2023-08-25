use std::io;
use std::io::Read;
use std::fs::File;

pub fn read(file_path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}
