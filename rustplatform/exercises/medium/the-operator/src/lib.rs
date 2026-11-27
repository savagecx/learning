fn parse_two_numbers(a: &str, b: &str) -> Result<(i32, i32), String> {
    Ok((
        a.parse::<i32>().map_err(|e| e.to_string())?,
        b.parse::<i32>().map_err(|e| e.to_string())?,
    ))
}

fn sum_first_two(numbers: &[i32]) -> Option<i32> {
    Some(numbers.first()? + numbers.get(1)?)
}

fn divide_parsed(a: &str, b: &str) -> Result<i32, String> {
    let (a, b) = parse_two_numbers(a, b)?;
    match b {
        0 => Err("Division by zero".to_string()),
        _ => Ok(a / b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_two_numbers_valid() {
        assert_eq!(parse_two_numbers("10", "20"), Ok((10, 20)));
    }

    #[test]
    fn test_parse_two_numbers_negative() {
        assert_eq!(parse_two_numbers("-5", "10"), Ok((-5, 10)));
    }

    #[test]
    fn test_parse_two_numbers_first_invalid() {
        assert!(parse_two_numbers("abc", "20").is_err());
    }

    #[test]
    fn test_parse_two_numbers_second_invalid() {
        assert!(parse_two_numbers("10", "abc").is_err());
    }

    #[test]
    fn test_sum_first_two_valid() {
        assert_eq!(sum_first_two(&[1, 2, 3]), Some(3));
    }

    #[test]
    fn test_sum_first_two_exact() {
        assert_eq!(sum_first_two(&[5, 10]), Some(15));
    }

    #[test]
    fn test_sum_first_two_one_element() {
        assert_eq!(sum_first_two(&[1]), None);
    }

    #[test]
    fn test_sum_first_two_empty() {
        assert_eq!(sum_first_two(&[]), None);
    }

    #[test]
    fn test_divide_parsed_valid() {
        assert_eq!(divide_parsed("10", "2"), Ok(5));
    }

    #[test]
    fn test_divide_parsed_negative() {
        assert_eq!(divide_parsed("-10", "2"), Ok(-5));
    }

    #[test]
    fn test_divide_parsed_division_by_zero() {
        assert!(divide_parsed("10", "0").is_err());
    }

    #[test]
    fn test_divide_parsed_invalid_first() {
        assert!(divide_parsed("abc", "2").is_err());
    }

    #[test]
    fn test_divide_parsed_invalid_second() {
        assert!(divide_parsed("10", "xyz").is_err());
    }
}
