fn grep<'a>(text: &'a str, pattern: &str, ignore_case: bool, invert: bool) -> Vec<&'a str> {
    text.lines()
        .filter(|line| {
            let found = if ignore_case {
                line.to_lowercase()
                    .contains(pattern.to_lowercase().as_str())
            } else {
                line.contains(pattern)
            };

            found != invert
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_match() {
        assert_eq!(grep("foo\nbar\nFOObar", "foo", false, false), vec!["foo"]);
    }

    #[test]
    fn test_ignore_case() {
        assert_eq!(
            grep("foo\nbar\nFOObar", "foo", true, false),
            vec!["foo", "FOObar"]
        );
    }

    #[test]
    fn test_invert() {
        assert_eq!(
            grep("foo\nbar\nFOObar", "foo", false, true),
            vec!["bar", "FOObar"]
        );
    }

    #[test]
    fn test_no_match() {
        assert_eq!(grep("foo\nbar", "xyz", false, false), Vec::<&str>::new());
    }
}
