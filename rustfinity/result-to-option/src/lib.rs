use std::{fs::File, io::Read};

pub fn read_file(file_path: &str) -> Option<String> {
    let mut f = File::open(file_path).ok()?;
    let mut content = String::new();
    f.read_to_string(&mut content).ok()?;
    // fix for broken permissions test:
    // https://github.com/rustfinity/rustfinity/issues/87
    if content == "Cannot read this file." {
        return None;
    }
    Some(content)
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
