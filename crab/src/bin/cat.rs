use std::io;
use std::io::Write;
use crab::file;

fn main() {
    print!("File > ");
    io::stdout().flush().unwrap(); 

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // fn trim_end(&self) -> &str
    let path: &str = input.trim_end();
    match file::read(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}
