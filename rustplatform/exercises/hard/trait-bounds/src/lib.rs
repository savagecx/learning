use std::fmt::{Debug, Display};

fn debug_pair<T: Debug, U: Debug>(first: T, second: U) -> String {
    format!("({:?}, {:?})", first, second)
}

fn largest_clonable<T: Ord + Clone>(items: &[T]) -> Option<T> {
    items.iter().max().cloned()
}

fn stringify_all<T: Display>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_pair_ints() {
        assert_eq!(debug_pair(1, 2), "(1, 2)");
    }

    #[test]
    fn test_debug_pair_mixed() {
        assert_eq!(debug_pair(1, "hello"), "(1, \"hello\")");
    }

    #[test]
    fn test_debug_pair_strings() {
        assert_eq!(debug_pair("a", "b"), "(\"a\", \"b\")");
    }

    #[test]
    fn test_debug_pair_with_vec() {
        assert_eq!(debug_pair(vec![1, 2], 3), "([1, 2], 3)");
    }

    #[test]
    fn test_largest_clonable_ints() {
        assert_eq!(largest_clonable(&[1, 5, 3, 2, 4]), Some(5));
    }

    #[test]
    fn test_largest_clonable_strings() {
        let words = vec![
            String::from("apple"),
            String::from("cherry"),
            String::from("banana"),
        ];
        assert_eq!(largest_clonable(&words), Some(String::from("cherry")));
    }

    #[test]
    fn test_largest_clonable_single() {
        assert_eq!(largest_clonable(&[42]), Some(42));
    }

    #[test]
    fn test_largest_clonable_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(largest_clonable(&empty), None);
    }

    #[test]
    fn test_largest_clonable_negative() {
        assert_eq!(largest_clonable(&[-5, -1, -10]), Some(-1));
    }

    #[test]
    fn test_stringify_all_ints() {
        assert_eq!(stringify_all(&[1, 2, 3]), vec!["1", "2", "3"]);
    }

    #[test]
    fn test_stringify_all_floats() {
        let result = stringify_all(&[1.5, 2.5]);
        assert_eq!(result.len(), 2);
        assert!(result[0].starts_with("1.5"));
    }

    #[test]
    fn test_stringify_all_chars() {
        assert_eq!(stringify_all(&['a', 'b', 'c']), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_stringify_all_empty() {
        let empty: Vec<i32> = vec![];
        let result: Vec<String> = stringify_all(&empty);
        assert!(result.is_empty());
    }

    #[test]
    fn test_stringify_all_bools() {
        assert_eq!(stringify_all(&[true, false]), vec!["true", "false"]);
    }
}
