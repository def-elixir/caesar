use std::io;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    let reversed = reverse(&text);
    println!("{}", reversed);
}

fn reverse(text: &str) -> String {
    let reversed_text = text.chars().rev().collect::<String>();
    reversed_text
}
