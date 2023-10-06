use std::process;
use std::{error::Error, fs::File, io::BufReader};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    item_id: i32,
    name: String,
    description: Option<String>,
}

fn load_json() -> Result<(), Box<dyn Error>> {
    let file = File::open("./file/item.json")?;
    let reader = BufReader::new(file);
    let item_list: Vec<Item> = serde_json::from_reader(reader)?;
    println!("{:?}", &item_list);
    Ok(())
}

fn write_json() -> Result<(), Box<dyn Error>> {
    // let mut item_list: Vec<Item> = Vec::new();
    // item_list.push(
    //     Item { item_id: 1, name: String::from("Mike"), description: Some(String::from("So cute!"))}
    // );
    let file = File::create("./file/item.json")?;
    let item_list: Vec<Item> = vec![
        Item { item_id: 1, name: String::from("Mike"), description: Some(String::from("So cute!"))}
    ];
    serde_json::to_writer_pretty(file, &item_list)?;
    Ok(())
}

fn main() {
    if let Err(e) = load_json() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
