#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<[i32; 2]> for Point {
    fn from(arr: [i32; 2]) -> Self {
        Point {
            x: arr[0],
            y: arr[1],
        }
    }
}

#[derive(Debug, PartialEq)]
struct Celsius(f64);

#[derive(Debug, PartialEq)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 1.8 + 32.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from_tuple() {
        let p = Point::from((3, 4));
        assert_eq!(p, Point { x: 3, y: 4 });
    }

    #[test]
    fn test_point_from_tuple_negative() {
        let p = Point::from((-1, -2));
        assert_eq!(p, Point { x: -1, y: -2 });
    }

    #[test]
    fn test_point_into_from_tuple() {
        let p: Point = (5, 6).into();
        assert_eq!(p, Point { x: 5, y: 6 });
    }

    #[test]
    fn test_point_from_array() {
        let p = Point::from([7, 8]);
        assert_eq!(p, Point { x: 7, y: 8 });
    }

    #[test]
    fn test_point_into_from_array() {
        let p: Point = [9, 10].into();
        assert_eq!(p, Point { x: 9, y: 10 });
    }

    #[test]
    fn test_celsius_to_fahrenheit_freezing() {
        let c = Celsius(0.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_fahrenheit_boiling() {
        let c = Celsius(100.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_fahrenheit_body_temp() {
        let c = Celsius(37.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - 98.6).abs() < 0.1);
    }

    #[test]
    fn test_celsius_to_fahrenheit_into() {
        let c = Celsius(25.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 77.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_fahrenheit_negative() {
        let c = Celsius(-40.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - (-40.0)).abs() < 0.001);
    }
}
