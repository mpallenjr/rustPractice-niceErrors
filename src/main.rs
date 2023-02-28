
#![allow(unused)]
fn main() {
let result = std::fs::read_to_string("text.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
}
