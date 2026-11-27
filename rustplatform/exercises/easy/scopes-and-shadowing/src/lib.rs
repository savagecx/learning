fn scope_and_shadowing() -> (i32, i32, String, String) {
    let x = 5; // Fix the error here, take a look at line 10 and the expected value in the test.
    let y = _; // Fill in the blank, take a look at line 10, we need a mutable reference.
    let s1 = String::from("outer"); // Fix the error here, take a look at the expected result in the test.

    {
        let x = 10; // Note: x from the outer scope is shadowed by the x in the inner scope while the inner scope exists.
        let z = &x; // z is a reference to x in the inner scope.

        *y = *y * _; // Fill in the blank with a number literal.
        let s2 = _; // Fill in the blank.

        // Add code here to modify s1. Hint: look at the expected return value in the test.

        return (*y, *z, s1, s2); // Don't change this line.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_and_shadowing() {
        assert_eq!(
            scope_and_shadowing(),
            (
                25,
                10,
                String::from("outer-modified"),
                String::from("inner")
            )
        );
    }
}
