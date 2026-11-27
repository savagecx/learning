struct User {
    name: String,
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    // Your code here
    todo!()
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    // Your code here
    todo!()
}

fn get_username(user: Option<User>) -> Option<String> {
    // Your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_even_found() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_first_even(&[2, 4, 6]), Some(2));
        assert_eq!(find_first_even(&[1, 2, 3]), Some(2));
    }

    #[test]
    fn test_find_first_even_not_found() {
        assert_eq!(find_first_even(&[1, 3, 5, 7]), None);
        assert_eq!(find_first_even(&[]), None);
    }

    #[test]
    fn test_safe_divide_success() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(9, 3), Some(3));
        assert_eq!(safe_divide(0, 5), Some(0));
    }

    #[test]
    fn test_safe_divide_by_zero() {
        assert_eq!(safe_divide(10, 0), None);
        assert_eq!(safe_divide(0, 0), None);
    }

    #[test]
    fn test_get_username_some() {
        let user = Some(User {
            name: "Alice".to_string(),
        });
        assert_eq!(get_username(user), Some("Alice".to_string()));
    }

    #[test]
    fn test_get_username_none() {
        assert_eq!(get_username(None), None);
    }
}
