fn safe_sqrt(n: f64) -> Result<f64, String> {
    // TODO: Return Ok(sqrt) if n >= 0, else Err("negative input")
    match n >= 0_f64 {
        true => Ok(n.sqrt()),
        false => Err("negative input".to_string()),
    }
}

fn validate_positive(n: i32) -> Result<u32, String> {
    match n > 0 {
        true => Ok(n as u32),
        false => Err("must be positive".to_string()),
    }
}

fn parse_positive(s: &str) -> Result<u32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Error: {}", e))
        .and_then(validate_positive)
}

fn chain_operations(s: &str) -> Result<f64, String> {
    parse_positive(s).and_then(|n| safe_sqrt(n as f64))
}

fn result_to_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_sqrt_positive() {
        assert_eq!(safe_sqrt(4.0), Ok(2.0));
        assert_eq!(safe_sqrt(9.0), Ok(3.0));
    }

    #[test]
    fn test_safe_sqrt_zero() {
        assert_eq!(safe_sqrt(0.0), Ok(0.0));
    }

    #[test]
    fn test_safe_sqrt_negative() {
        assert_eq!(safe_sqrt(-1.0), Err("negative input".to_string()));
    }

    #[test]
    fn test_parse_positive_valid() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("1"), Ok(1));
    }

    #[test]
    fn test_parse_positive_zero() {
        assert!(parse_positive("0").is_err());
    }

    #[test]
    fn test_parse_positive_negative() {
        assert!(parse_positive("-5").is_err());
    }

    #[test]
    fn test_parse_positive_invalid() {
        assert!(parse_positive("abc").is_err());
    }

    #[test]
    fn test_chain_operations_valid() {
        assert_eq!(chain_operations("16"), Ok(4.0));
        assert_eq!(chain_operations("25"), Ok(5.0));
    }

    #[test]
    fn test_chain_operations_negative() {
        assert!(chain_operations("-16").is_err());
    }

    #[test]
    fn test_chain_operations_invalid() {
        assert!(chain_operations("abc").is_err());
    }

    #[test]
    fn test_result_to_option_ok() {
        assert_eq!(result_to_option(Ok(42)), Some(42));
    }

    #[test]
    fn test_result_to_option_err() {
        assert_eq!(result_to_option(Err("error".to_string())), None);
    }
}
