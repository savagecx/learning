fn double_number(n: i32) -> (i32, i32) {
    (n, n * 2)
}

fn clone_and_modify(items: &Vec<i32>) -> Vec<i32> {
    items.iter().map(|x| x + 100).collect()
}

fn sum_of_copies(a: i32, b: i32, c: i32) -> i32 {
    (a + b + c) * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_number() {
        let (orig, doubled) = double_number(21);
        assert_eq!(orig, 21);
        assert_eq!(doubled, 42);
    }

    #[test]
    fn test_double_number_zero() {
        let (orig, doubled) = double_number(0);
        assert_eq!(orig, 0);
        assert_eq!(doubled, 0);
    }

    #[test]
    fn test_double_number_negative() {
        let (orig, doubled) = double_number(-5);
        assert_eq!(orig, -5);
        assert_eq!(doubled, -10);
    }

    #[test]
    fn test_clone_and_modify() {
        let numbers = vec![1, 2, 3];
        let modified = clone_and_modify(&numbers);
        assert_eq!(numbers, vec![1, 2, 3]);
        assert_eq!(modified, vec![101, 102, 103]);
    }

    #[test]
    fn test_clone_and_modify_empty() {
        let numbers: Vec<i32> = vec![];
        let modified = clone_and_modify(&numbers);
        assert_eq!(modified, Vec::<i32>::new());
    }

    #[test]
    fn test_clone_and_modify_single() {
        let numbers = vec![50];
        let modified = clone_and_modify(&numbers);
        assert_eq!(numbers, vec![50]);
        assert_eq!(modified, vec![150]);
    }

    #[test]
    fn test_sum_of_copies() {
        assert_eq!(sum_of_copies(1, 2, 3), 12);
    }

    #[test]
    fn test_sum_of_copies_zeros() {
        assert_eq!(sum_of_copies(0, 0, 0), 0);
    }

    #[test]
    fn test_sum_of_copies_same_value() {
        assert_eq!(sum_of_copies(5, 5, 5), 30);
    }
}
