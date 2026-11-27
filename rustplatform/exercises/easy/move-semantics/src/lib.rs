fn take_ownership(items: Vec<String>) -> usize {
    // TODO: Return the length of the vector
    // The vector is moved into this function and dropped when it returns
    todo!()
}

fn move_and_return(text: String) -> String {
    // TODO: Return the text wrapped in brackets: "[text]"
    // Ownership moves in and then back out
    todo!()
}

fn swap_ownership(a: String, b: String) -> (String, String) {
    // TODO: Return the strings in swapped order: (b, a)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership_returns_length() {
        let items = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        assert_eq!(take_ownership(items), 3);
    }

    #[test]
    fn test_take_ownership_empty_vec() {
        let items: Vec<String> = vec![];
        assert_eq!(take_ownership(items), 0);
    }

    #[test]
    fn test_move_and_return_wraps_text() {
        let text = String::from("hello");
        assert_eq!(move_and_return(text), "[hello]");
    }

    #[test]
    fn test_move_and_return_empty_string() {
        let text = String::from("");
        assert_eq!(move_and_return(text), "[]");
    }

    #[test]
    fn test_swap_ownership() {
        let (x, y) = swap_ownership("first".to_string(), "second".to_string());
        assert_eq!(x, "second");
        assert_eq!(y, "first");
    }

    #[test]
    fn test_swap_ownership_same_values() {
        let (x, y) = swap_ownership("same".to_string(), "same".to_string());
        assert_eq!(x, "same");
        assert_eq!(y, "same");
    }
}
