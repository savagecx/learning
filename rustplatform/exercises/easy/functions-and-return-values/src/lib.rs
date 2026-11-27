fn add_and_double(a: i32, b: i32) -> i32 {
    // TODO:
    // - Add a and b
    // - Multiply the result by 2
    // - Return the final result (no `return` needed if it's the last expression)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_double_positive() {
        assert_eq!(add_and_double(2, 3), 10);
    }

    #[test]
    fn test_add_and_double_mixed() {
        assert_eq!(add_and_double(-1, 4), 6);
    }
}
