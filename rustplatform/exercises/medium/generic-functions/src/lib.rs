fn first_and_last<T>(items: &[T]) -> Option<(&T, &T)> {
    if items.is_empty() {
        return None;
    }

    Some((items.first().unwrap(), items.last().unwrap()))
}

fn find_max<T: Ord>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }

    items.iter().max()
}

fn count_matches<T: PartialEq>(items: &[T], target: &T) -> usize {
    items.iter().filter(|&item| item.eq(target)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last_numbers() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(first_and_last(&nums), Some((&1, &5)));
    }

    #[test]
    fn test_first_and_last_strings() {
        let words = vec!["hello", "world", "rust"];
        assert_eq!(first_and_last(&words), Some((&"hello", &"rust")));
    }

    #[test]
    fn test_first_and_last_single() {
        let single = vec![42];
        assert_eq!(first_and_last(&single), Some((&42, &42)));
    }

    #[test]
    fn test_first_and_last_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(first_and_last(&empty), None);
    }

    #[test]
    fn test_find_max_numbers() {
        let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(find_max(&nums), Some(&9));
    }

    #[test]
    fn test_find_max_strings() {
        let words = vec!["apple", "banana", "cherry"];
        assert_eq!(find_max(&words), Some(&"cherry"));
    }

    #[test]
    fn test_find_max_single() {
        let single = vec![42];
        assert_eq!(find_max(&single), Some(&42));
    }

    #[test]
    fn test_find_max_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(find_max(&empty), None);
    }

    #[test]
    fn test_count_matches_found() {
        let nums = vec![1, 2, 2, 3, 2, 4];
        assert_eq!(count_matches(&nums, &2), 3);
    }

    #[test]
    fn test_count_matches_not_found() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(count_matches(&nums, &5), 0);
    }

    #[test]
    fn test_count_matches_strings() {
        let words = vec!["a", "b", "a", "c", "a"];
        assert_eq!(count_matches(&words, &"a"), 3);
    }

    #[test]
    fn test_count_matches_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(count_matches(&empty, &1), 0);
    }

    #[test]
    fn test_count_matches_all() {
        let same = vec![7, 7, 7, 7];
        assert_eq!(count_matches(&same, &7), 4);
    }
}
