fn is_palindrome(s: &str) -> bool {
    let cleaned = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    cleaned == cleaned.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("madam"));
        assert!(is_palindrome("level"));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("world"));
        assert!(!is_palindrome("rust"));
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_palindrome_with_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(is_palindrome("No 'x' in Nixon"));
    }

    #[test]
    fn test_empty_and_single() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
    }
}
