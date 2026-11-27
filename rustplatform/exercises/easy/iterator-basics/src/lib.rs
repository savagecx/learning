// Sum all elements in the slice using .iter()
fn sum_all(v: &[i32]) -> i32 {
    v.iter().sum()
}

// Double each element in place using .iter_mut()
fn double_all(v: &mut [i32]) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

// Convert Vec<i32> to Vec<String> using .into_iter()
// Hint: Use .map() and .collect()
fn into_strings(v: Vec<i32>) -> Vec<String> {
    v.into_iter().map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_all_basic() {
        assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_sum_all_empty() {
        assert_eq!(sum_all(&[]), 0);
    }

    #[test]
    fn test_sum_all_single() {
        assert_eq!(sum_all(&[42]), 42);
    }

    #[test]
    fn test_sum_all_negative() {
        assert_eq!(sum_all(&[-1, -2, 3]), 0);
    }

    #[test]
    fn test_sum_all_large() {
        let v: Vec<i32> = (1..=100).collect();
        assert_eq!(sum_all(&v), 5050);
    }

    #[test]
    fn test_double_all_basic() {
        let mut nums = vec![1, 2, 3];
        double_all(&mut nums);
        assert_eq!(nums, vec![2, 4, 6]);
    }

    #[test]
    fn test_double_all_empty() {
        let mut nums: Vec<i32> = vec![];
        double_all(&mut nums);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_double_all_negative() {
        let mut nums = vec![-1, 0, 1];
        double_all(&mut nums);
        assert_eq!(nums, vec![-2, 0, 2]);
    }

    #[test]
    fn test_double_all_twice() {
        let mut nums = vec![1, 2, 3];
        double_all(&mut nums);
        double_all(&mut nums);
        assert_eq!(nums, vec![4, 8, 12]);
    }

    #[test]
    fn test_into_strings_basic() {
        assert_eq!(into_strings(vec![1, 2, 3]), vec!["1", "2", "3"]);
    }

    #[test]
    fn test_into_strings_empty() {
        let result: Vec<String> = into_strings(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_into_strings_negative() {
        assert_eq!(into_strings(vec![-1, 0, 1]), vec!["-1", "0", "1"]);
    }

    #[test]
    fn test_into_strings_large_numbers() {
        assert_eq!(
            into_strings(vec![1000000, 2000000]),
            vec!["1000000", "2000000"]
        );
    }
}
