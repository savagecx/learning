fn count_words(text: &String) -> usize {
    // TODO: Count words in the text (split on whitespace)
    // Hint: String has methods for splitting — check the std::string docs
    text.split_whitespace().count()
}

fn find_longest(words: &Vec<String>) -> &String {
    words.iter().max_by_key(|word| word.len()).unwrap()
}

fn contains_value(items: &Vec<i32>, target: &i32) -> bool {
    items.contains(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_basic() {
        let text = String::from("hello world from rust");
        assert_eq!(count_words(&text), 4);
    }

    #[test]
    fn test_count_words_single() {
        let text = String::from("hello");
        assert_eq!(count_words(&text), 1);
    }

    #[test]
    fn test_count_words_empty() {
        let text = String::from("");
        assert_eq!(count_words(&text), 0);
    }

    #[test]
    fn test_count_words_extra_spaces() {
        let text = String::from("  hello   world  ");
        assert_eq!(count_words(&text), 2);
    }

    #[test]
    fn test_find_longest() {
        let words = vec![
            "short".to_string(),
            "medium".to_string(),
            "longest".to_string(),
        ];
        assert_eq!(find_longest(&words), "longest");
    }

    #[test]
    fn test_find_longest_first_is_longest() {
        let words = vec![
            "longest".to_string(),
            "short".to_string(),
            "medium".to_string(),
        ];
        assert_eq!(find_longest(&words), "longest");
    }

    #[test]
    fn test_find_longest_single() {
        let words = vec!["only".to_string()];
        assert_eq!(find_longest(&words), "only");
    }

    #[test]
    fn test_contains_value_found() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(contains_value(&numbers, &3));
    }

    #[test]
    fn test_contains_value_not_found() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert!(!contains_value(&numbers, &10));
    }

    #[test]
    fn test_contains_value_empty() {
        let numbers: Vec<i32> = vec![];
        assert!(!contains_value(&numbers, &1));
    }

    #[test]
    fn test_contains_value_first_element() {
        let numbers = vec![42, 1, 2];
        assert!(contains_value(&numbers, &42));
    }
}
