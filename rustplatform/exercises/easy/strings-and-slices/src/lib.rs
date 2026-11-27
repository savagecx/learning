fn first_word(s: &str) -> &str {
    // TODO: Return substring up to the first space or whole string if none
    todo!()
}

fn last_n(s: &str, n: usize) -> &str {
    // TODO: Return last n characters of s (clamp n to length)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rustacean"), "rustacean");
    }

    #[test]
    fn test_last_n() {
        assert_eq!(last_n("abcdef", 3), "def");
        assert_eq!(last_n("hi", 5), "hi");
    }
}
