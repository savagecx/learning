fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_reverse_single_char() {
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_reverse_palindrome() {
        assert_eq!(reverse_string("racecar"), "racecar");
    }

    #[test]
    fn test_reverse_with_spaces() {
        assert_eq!(reverse_string("hi there"), "ereht ih");
    }
}
