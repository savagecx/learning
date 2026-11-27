mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
}

mod calc {
    use std::f64::consts::PI;

    use super::shapes::{Circle, Rectangle};

    pub fn circle_area(c: &Circle) -> f64 {
        PI * c.radius * c.radius
    }

    pub fn rect_area(r: &Rectangle) -> f64 {
        r.width * r.height
    }
}

pub use calc::circle_area;
pub use calc::rect_area;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = shapes::Circle { radius: 1.0 };
        assert!((circle_area(&c) - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_circle_area_radius_2() {
        let c = shapes::Circle { radius: 2.0 };
        assert!((circle_area(&c) - 4.0 * std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_rect_area() {
        let r = shapes::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(rect_area(&r), 12.0);
    }

    #[test]
    fn test_rect_area_square() {
        let r = shapes::Rectangle {
            width: 5.0,
            height: 5.0,
        };
        assert_eq!(rect_area(&r), 25.0);
    }

    #[test]
    fn test_reexport_accessible() {
        // Verify we can access via re-export (not calc::)
        let c = shapes::Circle { radius: 1.0 };
        let _ = circle_area(&c);

        let r = shapes::Rectangle {
            width: 1.0,
            height: 1.0,
        };
        let _ = rect_area(&r);
    }
}
