use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Person {
    name: String,
    age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (age years old)"
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_display_positive() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(format!("{}", p), "(3, 4)");
    }

    #[test]
    fn test_point_display_negative() {
        let p = Point { x: -1, y: -5 };
        assert_eq!(format!("{}", p), "(-1, -5)");
    }

    #[test]
    fn test_point_display_zero() {
        let p = Point { x: 0, y: 0 };
        assert_eq!(format!("{}", p), "(0, 0)");
    }

    #[test]
    fn test_point_to_string() {
        let p = Point { x: 10, y: 20 };
        let s: String = p.to_string();
        assert_eq!(s, "(10, 20)");
    }

    #[test]
    fn test_person_display() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
        };
        assert_eq!(format!("{}", person), "Alice (30 years old)");
    }

    #[test]
    fn test_person_display_young() {
        let person = Person {
            name: String::from("Bob"),
            age: 5,
        };
        assert_eq!(format!("{}", person), "Bob (5 years old)");
    }

    #[test]
    fn test_person_to_string() {
        let person = Person {
            name: String::from("Charlie"),
            age: 25,
        };
        assert_eq!(person.to_string(), "Charlie (25 years old)");
    }
}
