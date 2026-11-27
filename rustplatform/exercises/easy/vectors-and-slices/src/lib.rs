fn sum_slice(v: &[i32]) -> i32 {
    // TODO: Return the sum of all items in v
    todo!()
}

fn square_in_place(v: &mut [i32]) {
    // TODO: Square each element of v in-place
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_slice_empty() {
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn test_sum_slice_nonempty() {
        let v = vec![1, 2, 3];
        assert_eq!(sum_slice(&v[..]), 6);
    }

    #[test]
    fn test_square_in_place() {
        let mut nums = vec![1, 2, 3];
        square_in_place(&mut nums[..]);
        assert_eq!(nums, vec![1, 4, 9]);
        assert_eq!(sum_slice(&nums[..]), 14);
    }
}
