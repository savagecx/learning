use std::fmt;

// TODO: Add #[derive(Debug)] to this struct
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Temperature {
    value: i32,
    unit: char,
}

// TODO: Implement Debug for Temperature manually
// Format should be: "Temperature(VALUE UNIT)" e.g., "Temperature(25 C)"
impl fmt::Debug for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Your code here
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_debug_derived() {
        let red = Color { r: 255, g: 0, b: 0 };
        let debug_str = format!("{:?}", red);
        assert!(debug_str.contains("255"));
        assert!(debug_str.contains("Color"));
    }

    #[test]
    fn test_color_debug_black() {
        let black = Color { r: 0, g: 0, b: 0 };
        let debug_str = format!("{:?}", black);
        assert!(debug_str.contains("Color"));
    }

    #[test]
    fn test_temperature_debug_celsius() {
        let temp = Temperature {
            value: 25,
            unit: 'C',
        };
        assert_eq!(format!("{:?}", temp), "Temperature(25 C)");
    }

    #[test]
    fn test_temperature_debug_fahrenheit() {
        let temp = Temperature {
            value: 77,
            unit: 'F',
        };
        assert_eq!(format!("{:?}", temp), "Temperature(77 F)");
    }

    #[test]
    fn test_temperature_debug_negative() {
        let temp = Temperature {
            value: -10,
            unit: 'C',
        };
        assert_eq!(format!("{:?}", temp), "Temperature(-10 C)");
    }

    #[test]
    fn test_temperature_debug_zero() {
        let temp = Temperature {
            value: 0,
            unit: 'K',
        };
        assert_eq!(format!("{:?}", temp), "Temperature(0 K)");
    }
}
