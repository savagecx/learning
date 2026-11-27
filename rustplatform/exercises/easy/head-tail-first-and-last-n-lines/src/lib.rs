fn head(text: &str, n: usize) -> Vec<&str> {
    text.lines().take(n).collect()
}

fn tail(text: &str, n: usize) -> Vec<&str> {
    let mut tail: Vec<&str> = text.lines().rev().take(n).collect();
    tail.reverse();
    tail
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_first_n_lines() {
        assert_eq!(head("a\nb\nc\nd\ne", 3), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_head_n_larger_than_input() {
        assert_eq!(head("a\nb", 5), vec!["a", "b"]);
    }

    #[test]
    fn test_head_zero() {
        assert_eq!(head("a\nb\nc", 0), Vec::<&str>::new());
    }

    #[test]
    fn test_tail_last_n_lines() {
        assert_eq!(tail("a\nb\nc\nd\ne", 2), vec!["d", "e"]);
    }

    #[test]
    fn test_tail_n_larger_than_input() {
        assert_eq!(tail("a\nb", 5), vec!["a", "b"]);
    }

    #[test]
    fn test_tail_zero() {
        assert_eq!(tail("a\nb\nc", 0), Vec::<&str>::new());
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(head("", 3), Vec::<&str>::new());
        assert_eq!(tail("", 2), Vec::<&str>::new());
    }
}
