fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;

    println!("file content: {}", content);
    Ok(())
}
