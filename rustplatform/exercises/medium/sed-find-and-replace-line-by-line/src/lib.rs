fn substitute(text: &str, from: &str, to: &str, global: bool) -> String {
    text.lines()
        .map(|line| match global {
            true => line.replace(from, to),
            false => line.replacen(from, to, 1),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_match_only() {
        assert_eq!(
            substitute("foo foo foo", "foo", "bar", false),
            "bar foo foo"
        );
    }

    #[test]
    fn test_global_replaces_all() {
        assert_eq!(substitute("foo foo foo", "foo", "bar", true), "bar bar bar");
    }

    #[test]
    fn test_first_match_per_line() {
        assert_eq!(substitute("a a\nb a", "a", "X", false), "X a\nb X");
    }

    #[test]
    fn test_global_per_line() {
        assert_eq!(substitute("a a\nb a", "a", "X", true), "X X\nb X");
    }

    #[test]
    fn test_no_match_unchanged() {
        assert_eq!(substitute("hello world", "xyz", "Z", false), "hello world");
        assert_eq!(substitute("hello world", "xyz", "Z", true), "hello world");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(substitute("", "a", "b", false), "");
        assert_eq!(substitute("", "a", "b", true), "");
    }
}
