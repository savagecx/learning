use std::collections::HashMap;

fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        counts
            .entry(word.to_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut output = counts.into_iter().collect::<Vec<_>>();
    output.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    output.iter().take(n).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_two() {
        assert_eq!(
            top_words("the cat the dog the", 2),
            vec![("the".to_string(), 3), ("cat".to_string(), 1)]
        );
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(
            top_words("a A a b B", 2),
            vec![("a".to_string(), 3), ("b".to_string(), 2)]
        );
    }

    #[test]
    fn test_ties_broken_alphabetically() {
        assert_eq!(
            top_words("pear apple pear apple", 2),
            vec![("apple".to_string(), 2), ("pear".to_string(), 2)]
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(top_words("x y z", 0), Vec::<(String, usize)>::new());
    }
}
