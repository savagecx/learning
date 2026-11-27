fn sort_lines(text: &str, numeric: bool) -> Vec<&str> {
    let mut lines: Vec<&str> = text.lines().collect();

    if numeric {
        lines.sort_by_key(|&line| line.parse::<u64>().unwrap_or(0));
    } else {
        lines.sort()
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_sort() {
        assert_eq!(
            sort_lines("banana\napple\ncherry", false),
            vec!["apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_numeric_sort() {
        assert_eq!(
            sort_lines("10\n9\n100\n2", true),
            vec!["2", "9", "10", "100"]
        );
    }

    #[test]
    fn test_lexical_differs_from_numeric() {
        assert_eq!(
            sort_lines("10\n9\n100\n2", false),
            vec!["10", "100", "2", "9"]
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(sort_lines("", false), Vec::<&str>::new());
    }
}
