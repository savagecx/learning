fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Implement conversion from Celsius to Fahrenheit
    todo!()
}

fn is_even(n: u32) -> bool {
    // Implement check for even number
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
    }
}
