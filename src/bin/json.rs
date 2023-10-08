use std::process;
use std::{error::Error, fs::File, io::BufReader};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    item_id: i32,
    name: String,
    description: Option<String>,
}

fn json_pretty_print<T>(data: &T) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let data_str = serde_json::to_string_pretty(data)?;
    println!("{}", data_str);

    Ok(())
}

fn load_json(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn write_json(path: &str) -> Result<(), Box<dyn Error>> {
    // let mut item_list: Vec<Item> = Vec::new();
    // item_list.push(
    //     Item { item_id: 1, name: String::from("Mike"), description: Some(String::from("So cute!"))}
    // );
    let file = File::create(path)?;
    let item_list: Vec<Item> = vec![
        Item { item_id: 1, name: String::from("Mike"), description: Some(String::from("So cute!"))},
        Item { item_id: 2, name: String::from("Pochi"), description: Some(String::from("So cute!"))}
    ];
    serde_json::to_writer_pretty(file, &item_list)?;
    Ok(())
}

fn remove_by_item_id(mut item_list: Vec<Item>, id: i32) -> Vec<Item> {
    item_list.retain(|item| item.item_id != id);

    item_list
}

fn run() -> Result<(), Box<dyn Error>>  {
    let reader = load_json("./file/item.json")?;
    let item_list: Vec<Item> = serde_json::from_reader(reader)?;
    let content = remove_by_item_id(item_list, 3);
    json_pretty_print(&content)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
