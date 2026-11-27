fn describe_types() -> String {
    // TODO:
    // - Declare an int, float, bool, char, and a tuple (int, &str) using type inference where possible.
    // - Use the `format!` macro to produce a String in the format:
    //   int: 42, float: 3.14, bool: true, char: Z, tuple: (7, "Rust")
    // - Remember: if the last expression in a function doesn’t end with a semicolon,
    //   it becomes the return value — no `return` keyword needed.

    format!(
        // fill in variables here
        ""
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_types() {
        let output = describe_types();
        assert!(output.contains("int: 42"));
        assert!(output.contains("float: 3.14"));
        assert!(output.contains("bool: true"));
        assert!(output.contains("char: Z"));
        assert!(output.contains("tuple: (7, \"Rust\")"));
    }
}