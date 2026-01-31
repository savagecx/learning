use std::error::Error;
use std::fmt;

// 1. Finish the definition
#[derive(Debug, PartialEq)]
pub enum ParsePercentageError {
    OutOfRange,
    InvalidInput,
}

impl fmt::Display for ParsePercentageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::OutOfRange => write!(f, "Percentage out of range"),
            Self::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl Error for ParsePercentageError {}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    match input.parse::<u8>() {
        Ok(num) => {
            if num <= 100 {
                return Ok(num);
            } else {
                return Err(ParsePercentageError::OutOfRange);
            }
        }
        Err(_) => return Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}
