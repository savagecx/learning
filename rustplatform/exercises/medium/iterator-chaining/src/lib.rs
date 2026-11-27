use std::collections::HashSet;

fn process_scores(scores: &[i32]) -> Vec<i32> {
    scores
        .iter()
        .filter(|&&score| score >= 60)
        .map(|score| (score + 10).min(100))
        .collect()
}

fn flatten_and_sort(nested: Vec<Vec<i32>>) -> Vec<i32> {
    let mut flattened: Vec<i32> = nested
        .into_iter()
        .flatten()
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect();
    flattened.sort();
    flattened
}

fn top_n(items: &[i32], n: usize) -> Vec<i32> {
    let mut sorted: Vec<i32> = items.to_vec();
    sorted.sort();
    sorted.reverse();
    sorted.into_iter().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_scores_basic() {
        assert_eq!(process_scores(&[55, 70, 95, 40]), vec![80, 100]);
    }

    #[test]
    fn test_process_scores_all_pass() {
        assert_eq!(process_scores(&[60, 70, 80]), vec![70, 80, 90]);
    }

    #[test]
    fn test_process_scores_none_pass() {
        let result: Vec<i32> = process_scores(&[10, 20, 30]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_process_scores_empty() {
        let result: Vec<i32> = process_scores(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_process_scores_cap_at_100() {
        assert_eq!(process_scores(&[91, 92, 100]), vec![100, 100, 100]);
    }

    #[test]
    fn test_process_scores_boundary() {
        assert_eq!(process_scores(&[59, 60, 61]), vec![70, 71]);
    }

    #[test]
    fn test_flatten_and_sort_basic() {
        assert_eq!(
            flatten_and_sort(vec![vec![3, 1], vec![2, 1]]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_flatten_and_sort_empty() {
        let result: Vec<i32> = flatten_and_sort(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_flatten_and_sort_empty_inner() {
        assert_eq!(
            flatten_and_sort(vec![vec![], vec![1, 2], vec![]]),
            vec![1, 2]
        );
    }

    #[test]
    fn test_flatten_and_sort_no_duplicates() {
        assert_eq!(
            flatten_and_sort(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_flatten_and_sort_all_duplicates() {
        assert_eq!(flatten_and_sort(vec![vec![1, 1], vec![1, 1]]), vec![1]);
    }

    #[test]
    fn test_top_n_basic() {
        assert_eq!(top_n(&[5, 2, 8, 1, 9], 3), vec![9, 8, 5]);
    }

    #[test]
    fn test_top_n_take_all() {
        assert_eq!(top_n(&[3, 1, 2], 5), vec![3, 2, 1]);
    }

    #[test]
    fn test_top_n_take_one() {
        assert_eq!(top_n(&[5, 2, 8, 1, 9], 1), vec![9]);
    }

    #[test]
    fn test_top_n_take_zero() {
        let result: Vec<i32> = top_n(&[1, 2, 3], 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_top_n_empty() {
        let result: Vec<i32> = top_n(&[], 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_top_n_with_duplicates() {
        assert_eq!(top_n(&[5, 5, 3, 3, 1], 3), vec![5, 5, 3]);
    }

    #[test]
    fn test_top_n_negative() {
        assert_eq!(top_n(&[-1, -5, -2], 2), vec![-1, -2]);
    }
}
