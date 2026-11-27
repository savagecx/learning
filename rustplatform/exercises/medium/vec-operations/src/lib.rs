fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = a.to_vec();
    merged.extend(b.iter());
    merged.sort();
    merged
}

fn chunk_vec(items: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    items.chunks(size).map(|x| x.into()).collect()
}

fn partition_by_predicate(items: Vec<i32>, threshold: i32) -> (Vec<i32>, Vec<i32>) {
    items.into_iter().partition(|&x| x < threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_basic() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_one_empty() {
        assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_sorted_both_empty() {
        assert_eq!(merge_sorted(&[], &[]), vec![]);
    }

    #[test]
    fn test_merge_sorted_duplicates() {
        assert_eq!(merge_sorted(&[1, 2, 3], &[2, 3, 4]), vec![1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_merge_sorted_unequal_lengths() {
        assert_eq!(merge_sorted(&[1], &[2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_chunk_vec_even() {
        assert_eq!(chunk_vec(vec![1, 2, 3, 4], 2), vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_chunk_vec_uneven() {
        assert_eq!(
            chunk_vec(vec![1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
    }

    #[test]
    fn test_chunk_vec_single_chunk() {
        assert_eq!(chunk_vec(vec![1, 2, 3], 5), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_chunk_vec_size_one() {
        assert_eq!(chunk_vec(vec![1, 2, 3], 1), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_chunk_vec_empty() {
        let result: Vec<Vec<i32>> = chunk_vec(vec![], 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_partition_by_predicate() {
        let (below, above) = partition_by_predicate(vec![1, 5, 2, 8, 3], 4);
        assert_eq!(below, vec![1, 2, 3]);
        assert_eq!(above, vec![5, 8]);
    }

    #[test]
    fn test_partition_all_below() {
        let (below, above) = partition_by_predicate(vec![1, 2, 3], 10);
        assert_eq!(below, vec![1, 2, 3]);
        assert_eq!(above, vec![]);
    }

    #[test]
    fn test_partition_all_above() {
        let (below, above) = partition_by_predicate(vec![10, 20, 30], 5);
        assert_eq!(below, vec![]);
        assert_eq!(above, vec![10, 20, 30]);
    }

    #[test]
    fn test_partition_empty() {
        let (below, above) = partition_by_predicate(vec![], 5);
        assert_eq!(below, vec![]);
        assert_eq!(above, vec![]);
    }
}
