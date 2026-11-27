fn cat_n(text: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(i, v)| format!("{}\t{}", i + 1, v))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_lines() {
        assert_eq!(cat_n("foo\nbar\nbaz"), "1\tfoo\n2\tbar\n3\tbaz");
    }

    #[test]
    fn test_single_line() {
        assert_eq!(cat_n("only"), "1\tonly");
    }

    #[test]
    fn test_blank_line_kept() {
        assert_eq!(cat_n("a\n\nb"), "1\ta\n2\t\n3\tb");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(cat_n(""), "");
    }
}
