use std::env;
use std::process;
use std::error::Error;
use caesar::utils::file;

struct Arguments {
    filename: String,
}

impl Arguments {
    fn new(mut args: std::env::Args) -> Result<Arguments, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Arguments { filename })
    }
}

fn main() {
    let args = Arguments::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing arguments error: {}", err);
        process::exit(1);
    });

    if let Err(e) = cat(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn cat(args: Arguments) -> Result<(), Box<dyn Error>> {
    let contents: String = file::read(&args.filename)?;
    println!("{}", contents);

    Ok(())
}
