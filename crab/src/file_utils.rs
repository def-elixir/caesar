use std::io;
use std::io::Read;
use std::fs::File;

pub fn read(filename: String) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;
    Ok(contents)
}
