fn uniq_c(text: &str) -> Vec<(usize, &str)> {
    let mut pairs = Vec::<(usize, &str)>::new();

    for line in text.lines() {
        match pairs.last_mut() {
            Some((count, last)) if line.eq(*last) => *count += 1,
            _ => pairs.push((1, line)),
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapses_runs() {
        assert_eq!(
            uniq_c("a\na\nb\nb\nb\na"),
            vec![(2, "a"), (3, "b"), (1, "a")]
        );
    }

    #[test]
    fn test_single_line() {
        assert_eq!(uniq_c("x"), vec![(1, "x")]);
    }

    #[test]
    fn test_no_adjacent_dupes() {
        assert_eq!(uniq_c("a\nb\na"), vec![(1, "a"), (1, "b"), (1, "a")]);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(uniq_c(""), Vec::<(usize, &str)>::new());
    }
}
