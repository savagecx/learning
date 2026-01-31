use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn sum_integers_from_file(file_path: &str) -> Result<i32, io::Error> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut total = 0;

    for line in f.lines() {
        let line = line?;
        total += line
            .parse::<i32>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
    }

    Ok(total)
}

// Example usage
pub fn main() {
    let file_path = "numbers.txt";

    match sum_integers_from_file(file_path) {
        Ok(sum) => println!("The sum is: {}", sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
