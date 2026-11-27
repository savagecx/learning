use std::ops::Add;

fn double_if_some(n: Option<i32>) -> Option<i32> {
    n.map(|val| val * 2)
}

fn get_or_default(opt: Option<String>, default: &str) -> String {
    opt.unwrap_or(default.to_string())
}

fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&n| n % 2 == 0).map(|n| n * 2)
}

fn parse_and_add(s: Option<&str>, add: i32) -> Option<i32> {
    // TODO: Parse the string to i32 and add the value
    // Hint: .and_then() to chain Option-returning operations
    // str.parse::<i32>().ok() converts Result to Option
    s.and_then(|str| str.parse::<i32>().ok()).map(|n| n + add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_if_some() {
        assert_eq!(double_if_some(Some(5)), Some(10));
        assert_eq!(double_if_some(Some(0)), Some(0));
        assert_eq!(double_if_some(Some(-3)), Some(-6));
        assert_eq!(double_if_some(None), None);
    }

    #[test]
    fn test_get_or_default_some() {
        assert_eq!(
            get_or_default(Some("hello".to_string()), "default"),
            "hello"
        );
    }

    #[test]
    fn test_get_or_default_none() {
        assert_eq!(get_or_default(None, "default"), "default");
    }

    #[test]
    fn test_get_or_default_empty_string() {
        assert_eq!(get_or_default(Some("".to_string()), "default"), "");
    }

    #[test]
    fn test_first_even_found() {
        assert_eq!(first_even(&[1, 3, 4, 5]), Some(8));
    }

    #[test]
    fn test_first_even_first_is_even() {
        assert_eq!(first_even(&[2, 4, 6]), Some(4));
    }

    #[test]
    fn test_first_even_not_found() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn test_first_even_empty() {
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn test_parse_and_add_valid() {
        assert_eq!(parse_and_add(Some("10"), 5), Some(15));
    }

    #[test]
    fn test_parse_and_add_invalid() {
        assert_eq!(parse_and_add(Some("abc"), 5), None);
    }

    #[test]
    fn test_parse_and_add_none() {
        assert_eq!(parse_and_add(None, 5), None);
    }

    #[test]
    fn test_parse_and_add_negative() {
        assert_eq!(parse_and_add(Some("-5"), 10), Some(5));
    }
}
