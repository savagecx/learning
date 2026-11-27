fn divide(a: i32, b: i32) -> Result<i32, String> {
    // your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(7, 2), Ok(3));
    }

    #[test]
    fn test_divide_zero() {
        assert_eq!(divide(10, 0), Err("division by zero".to_string()));
        assert_eq!(divide(-5, 0), Err("division by zero".to_string()));
    }
}
