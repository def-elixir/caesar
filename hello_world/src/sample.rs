use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

pub fn cat() {
    print!("File > ");
    io::stdout().flush().unwrap(); 

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // fn trim_end(&self) -> &str
    let path: &str = input.trim_end();
    match read_file(path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("{}", e),
    }
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}
