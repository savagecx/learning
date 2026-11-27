fn greet() -> String {
    // your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, Rustacean!");
    }
}
