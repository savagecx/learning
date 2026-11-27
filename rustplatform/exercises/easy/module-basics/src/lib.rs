// TODO: Create a `geometry` module with the required functions
// Hint: Use `pub mod geometry { ... }` to create a public module
// Hint: Use `std::f64::consts::PI` for the circle area

#[cfg(test)]
mod tests {
    use super::geometry;

    #[test]
    fn test_area_rectangle() {
        assert_eq!(geometry::area_rectangle(3.0, 4.0), 12.0);
    }

    #[test]
    fn test_area_rectangle_square() {
        assert_eq!(geometry::area_rectangle(5.0, 5.0), 25.0);
    }

    #[test]
    fn test_area_circle() {
        let area = geometry::area_circle(1.0);
        assert!((area - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_area_circle_radius_2() {
        let area = geometry::area_circle(2.0);
        assert!((area - 4.0 * std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_helpers_square() {
        assert_eq!(geometry::helpers::square(5.0), 25.0);
    }

    #[test]
    fn test_helpers_square_zero() {
        assert_eq!(geometry::helpers::square(0.0), 0.0);
    }

    #[test]
    fn test_helpers_square_negative() {
        assert_eq!(geometry::helpers::square(-3.0), 9.0);
    }
}
