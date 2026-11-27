// Return a Vec containing the square of each number
fn squares(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * x).collect()
}

// Return only the positive numbers (> 0)
fn only_positive(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&x| x > 0).copied().collect()
}

// Parse strings to i32, ignoring any that fail to parse
// Hint: You need to both transform and filter in one step
fn parse_numbers(v: &[&str]) -> Vec<i32> {
    v.iter().filter_map(|x| x.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares_basic() {
        assert_eq!(squares(&[1, 2, 3]), vec![1, 4, 9]);
    }

    #[test]
    fn test_squares_empty() {
        let result: Vec<i32> = squares(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_squares_negative() {
        assert_eq!(squares(&[-2, -1, 0, 1, 2]), vec![4, 1, 0, 1, 4]);
    }

    #[test]
    fn test_squares_single() {
        assert_eq!(squares(&[5]), vec![25]);
    }

    #[test]
    fn test_only_positive_basic() {
        assert_eq!(only_positive(&[-1, 0, 1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_only_positive_all_positive() {
        assert_eq!(only_positive(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_only_positive_none_positive() {
        let result: Vec<i32> = only_positive(&[-1, -2, 0]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_only_positive_empty() {
        let result: Vec<i32> = only_positive(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_numbers_basic() {
        assert_eq!(parse_numbers(&["1", "two", "3"]), vec![1, 3]);
    }

    #[test]
    fn test_parse_numbers_all_valid() {
        assert_eq!(parse_numbers(&["1", "2", "3"]), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_numbers_none_valid() {
        let result: Vec<i32> = parse_numbers(&["a", "b", "c"]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_numbers_negative() {
        assert_eq!(parse_numbers(&["-1", "0", "1"]), vec![-1, 0, 1]);
    }

    #[test]
    fn test_parse_numbers_empty() {
        let result: Vec<i32> = parse_numbers(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_numbers_whitespace() {
        assert_eq!(parse_numbers(&["1", " 2", "3 "]), vec![1]);
    }
}
