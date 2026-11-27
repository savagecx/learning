fn first_and_len(items: &Vec<i32>) -> (i32, usize) {
    (items[0], items.len())
}

fn push_and_get_first(items: &mut Vec<i32>, value: i32) -> i32 {
    items.push(value);
    items[0]
}

fn clone_and_extend(original: &Vec<String>, suffix: &str) -> Vec<String> {
    original.iter().map(|s| format!("{s}{suffix}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_len() {
        let nums = vec![10, 20, 30];
        let (first, len) = first_and_len(&nums);
        assert_eq!(first, 10);
        assert_eq!(len, 3);
    }

    #[test]
    fn test_first_and_len_single() {
        let nums = vec![42];
        let (first, len) = first_and_len(&nums);
        assert_eq!(first, 42);
        assert_eq!(len, 1);
    }

    #[test]
    fn test_first_and_len_negative() {
        let nums = vec![-5, 0, 5];
        let (first, len) = first_and_len(&nums);
        assert_eq!(first, -5);
        assert_eq!(len, 3);
    }

    #[test]
    fn test_push_and_get_first() {
        let mut items = vec![1, 2, 3];
        let first = push_and_get_first(&mut items, 4);
        assert_eq!(first, 1);
        assert_eq!(items, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_push_and_get_first_empty_start() {
        let mut items = vec![100];
        let first = push_and_get_first(&mut items, 200);
        assert_eq!(first, 100);
        assert_eq!(items, vec![100, 200]);
    }

    #[test]
    fn test_push_and_get_first_same_value() {
        let mut items = vec![5, 5, 5];
        let first = push_and_get_first(&mut items, 5);
        assert_eq!(first, 5);
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_clone_and_extend() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let extended = clone_and_extend(&words, "!");
        assert_eq!(extended, vec!["hello!", "world!"]);
        assert_eq!(words, vec!["hello", "world"]);
    }

    #[test]
    fn test_clone_and_extend_empty_suffix() {
        let words = vec!["test".to_string()];
        let extended = clone_and_extend(&words, "");
        assert_eq!(extended, vec!["test"]);
    }

    #[test]
    fn test_clone_and_extend_empty_vec() {
        let words: Vec<String> = vec![];
        let extended = clone_and_extend(&words, "!");
        assert_eq!(extended, Vec::<String>::new());
    }

    #[test]
    fn test_clone_and_extend_longer_suffix() {
        let words = vec!["a".to_string(), "b".to_string()];
        let extended = clone_and_extend(&words, "_suffix");
        assert_eq!(extended, vec!["a_suffix", "b_suffix"]);
    }
}
