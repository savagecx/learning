fn double_all(numbers: &mut Vec<i32>) {
    numbers.iter_mut().map(|x| *x *= 2).collect()
}

fn remove_negatives(numbers: &mut Vec<i32>) {
    numbers.retain(|&x| x >= 0);
}

fn append_reversed(dest: &mut String, source: &str) {
    dest.push_str(source.chars().rev().collect::<String>().as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_all() {
        let mut nums = vec![1, 2, 3];
        double_all(&mut nums);
        assert_eq!(nums, vec![2, 4, 6]);
    }

    #[test]
    fn test_double_all_empty() {
        let mut nums: Vec<i32> = vec![];
        double_all(&mut nums);
        assert_eq!(nums, Vec::<i32>::new());
    }

    #[test]
    fn test_double_all_negatives() {
        let mut nums = vec![-1, -2, 0];
        double_all(&mut nums);
        assert_eq!(nums, vec![-2, -4, 0]);
    }

    #[test]
    fn test_double_all_single() {
        let mut nums = vec![5];
        double_all(&mut nums);
        assert_eq!(nums, vec![10]);
    }

    #[test]
    fn test_remove_negatives() {
        let mut nums = vec![1, -2, 3, -4, 5];
        remove_negatives(&mut nums);
        assert_eq!(nums, vec![1, 3, 5]);
    }

    #[test]
    fn test_remove_negatives_all_negative() {
        let mut nums = vec![-1, -2, -3];
        remove_negatives(&mut nums);
        assert_eq!(nums, Vec::<i32>::new());
    }

    #[test]
    fn test_remove_negatives_none_negative() {
        let mut nums = vec![1, 2, 3];
        remove_negatives(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_negatives_with_zero() {
        let mut nums = vec![-1, 0, 1];
        remove_negatives(&mut nums);
        assert_eq!(nums, vec![0, 1]);
    }

    #[test]
    fn test_append_reversed() {
        let mut text = String::from("hello ");
        append_reversed(&mut text, "dlrow");
        assert_eq!(text, "hello world");
    }

    #[test]
    fn test_append_reversed_empty_source() {
        let mut text = String::from("hello");
        append_reversed(&mut text, "");
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_append_reversed_empty_dest() {
        let mut text = String::from("");
        append_reversed(&mut text, "tset");
        assert_eq!(text, "test");
    }

    #[test]
    fn test_append_reversed_palindrome() {
        let mut text = String::from("");
        append_reversed(&mut text, "level");
        assert_eq!(text, "level");
    }
}
