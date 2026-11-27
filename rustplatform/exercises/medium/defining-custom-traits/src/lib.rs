trait Drawable {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (u32, u32);
}

struct Circle {
    radius: u32,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("Circle(radius={})", self.radius)
    }

    fn bounding_box(&self) -> (u32, u32) {
        let x = 2 * self.radius;
        (x, x)
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("Rectangle({}x{})", self.width, self.height)
    }

    fn bounding_box(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_draw() {
        let circle = Circle { radius: 5 };
        assert_eq!(circle.draw(), "Circle(radius=5)");
    }

    #[test]
    fn test_circle_draw_zero() {
        let circle = Circle { radius: 0 };
        assert_eq!(circle.draw(), "Circle(radius=0)");
    }

    #[test]
    fn test_circle_bounding_box() {
        let circle = Circle { radius: 5 };
        assert_eq!(circle.bounding_box(), (10, 10));
    }

    #[test]
    fn test_circle_bounding_box_large() {
        let circle = Circle { radius: 100 };
        assert_eq!(circle.bounding_box(), (200, 200));
    }

    #[test]
    fn test_rectangle_draw() {
        let rect = Rectangle {
            width: 4,
            height: 3,
        };
        assert_eq!(rect.draw(), "Rectangle(4x3)");
    }

    #[test]
    fn test_rectangle_draw_square() {
        let rect = Rectangle {
            width: 5,
            height: 5,
        };
        assert_eq!(rect.draw(), "Rectangle(5x5)");
    }

    #[test]
    fn test_rectangle_bounding_box() {
        let rect = Rectangle {
            width: 4,
            height: 3,
        };
        assert_eq!(rect.bounding_box(), (4, 3));
    }

    #[test]
    fn test_drawable_as_trait_object() {
        let shapes: Vec<Box<dyn Drawable>> = vec![
            Box::new(Circle { radius: 2 }),
            Box::new(Rectangle {
                width: 3,
                height: 4,
            }),
        ];

        assert_eq!(shapes[0].draw(), "Circle(radius=2)");
        assert_eq!(shapes[1].draw(), "Rectangle(3x4)");
    }

    #[test]
    fn test_generic_function_with_drawable() {
        fn total_area(shapes: &[&dyn Drawable]) -> u32 {
            shapes
                .iter()
                .map(|s| {
                    let (w, h) = s.bounding_box();
                    w * h
                })
                .sum()
        }

        let circle = Circle { radius: 2 };
        let rect = Rectangle {
            width: 3,
            height: 4,
        };

        let shapes: Vec<&dyn Drawable> = vec![&circle, &rect];
        assert_eq!(total_area(&shapes), 16 + 12);
    }
}
