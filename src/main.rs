
#![allow(unused)]
fn main() {
let content = std::fs::read_to_string("text.txt").unwrap();

println!("file content: {}", content);
}
