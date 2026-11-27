use std::collections::HashSet;

fn unique_items(items: Vec<i32>) -> Vec<i32> {
    let mut unique: HashSet<i32> = HashSet::new();
    items
        .into_iter()
        .filter(|item| unique.insert(*item))
        .collect()
}

fn common_elements(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a: HashSet<i32> = a.iter().copied().collect();
    let b: HashSet<i32> = b.iter().copied().collect();

    let mut union: Vec<i32> = a.intersection(&b).copied().collect();
    union.sort();
    union
}

fn is_subset(subset: &[i32], superset: &[i32]) -> bool {
    let subset: HashSet<i32> = subset.iter().copied().collect();
    let superset: HashSet<i32> = superset.iter().copied().collect();

    subset.is_subset(&superset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_items() {
        assert_eq!(unique_items(vec![3, 1, 2, 1, 3]), vec![3, 1, 2]);
    }

    #[test]
    fn test_unique_items_all_unique() {
        assert_eq!(unique_items(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_unique_items_all_same() {
        assert_eq!(unique_items(vec![5, 5, 5]), vec![5]);
    }

    #[test]
    fn test_unique_items_empty() {
        assert_eq!(unique_items(vec![]), vec![]);
    }

    #[test]
    fn test_common_elements() {
        let common = common_elements(&[1, 2, 3, 4], &[3, 4, 5, 6]);
        assert_eq!(common, vec![3, 4]);
    }

    #[test]
    fn test_common_elements_none() {
        let common = common_elements(&[1, 2], &[3, 4]);
        assert_eq!(common, vec![]);
    }

    #[test]
    fn test_common_elements_all() {
        let common = common_elements(&[1, 2, 3], &[1, 2, 3]);
        assert_eq!(common, vec![1, 2, 3]);
    }

    #[test]
    fn test_common_elements_empty() {
        let common = common_elements(&[], &[1, 2, 3]);
        assert_eq!(common, vec![]);
    }

    #[test]
    fn test_is_subset_true() {
        assert!(is_subset(&[1, 2], &[1, 2, 3, 4]));
    }

    #[test]
    fn test_is_subset_false() {
        assert!(!is_subset(&[1, 5], &[1, 2, 3, 4]));
    }

    #[test]
    fn test_is_subset_equal() {
        assert!(is_subset(&[1, 2, 3], &[1, 2, 3]));
    }

    #[test]
    fn test_is_subset_empty() {
        assert!(is_subset(&[], &[1, 2, 3]));
    }
}
