use std::collections::HashMap;

fn word_count(s: &str) -> HashMap<String, usize> {
    let mut wordcount = HashMap::new();
    let words: Vec<&str> = s.split_whitespace().collect();

    for word in words {
        *wordcount
            .entry(
                word.to_lowercase()
                    .chars()
                    .filter(|c| !c.is_ascii_punctuation())
                    .collect::<String>(),
            )
            .or_insert(0) += 1;
    }

    wordcount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_count() {
        let counts = word_count("hello world");
        assert_eq!(counts.get("hello"), Some(&1));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_repeated_words() {
        let counts = word_count("hello hello hello");
        assert_eq!(counts.get("hello"), Some(&3));
    }

    #[test]
    fn test_case_insensitive() {
        let counts = word_count("Hello HELLO hello");
        assert_eq!(counts.get("hello"), Some(&3));
    }

    #[test]
    fn test_with_punctuation() {
        let counts = word_count("Hello, world! Hello.");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_empty_string() {
        let counts = word_count("");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_sentence() {
        let counts = word_count("The quick brown fox jumps over the lazy dog");
        assert_eq!(counts.get("the"), Some(&2));
        assert_eq!(counts.get("fox"), Some(&1));
        assert_eq!(counts.len(), 8);
    }
}
