struct Range {
    current: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let val = self.current;
        self.current += 1;

        Some(val)
    }
}

struct Fibonacci {
    current: u64,
    next: u64,
    remaining: usize,
}

impl Fibonacci {
    fn new(n: usize) -> Self {
        Self {
            current: 0,
            next: 1,
            remaining: n,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let current_fib = self.current;
        self.current = self.next;
        self.next += current_fib;
        self.remaining -= 1;

        Some(current_fib)
    }
}

struct Cycle<'a, T> {
    items: &'a [T],
    index: usize,
}

impl<'a, T> Cycle<'a, T> {
    fn new(items: &'a [T]) -> Self {
        Self { items, index: 0 }
    }
}

impl<'a, T> Iterator for Cycle<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let val = self.items.get(self.index);
        self.index = (self.index + 1) % self.items.len();
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_basic() {
        let r: Vec<i32> = Range::new(1, 4).collect();
        assert_eq!(r, vec![1, 2, 3]);
    }

    #[test]
    fn test_range_empty() {
        let r: Vec<i32> = Range::new(5, 5).collect();
        assert!(r.is_empty());
    }

    #[test]
    fn test_range_negative() {
        let r: Vec<i32> = Range::new(-2, 2).collect();
        assert_eq!(r, vec![-2, -1, 0, 1]);
    }

    #[test]
    fn test_range_reversed_empty() {
        let r: Vec<i32> = Range::new(5, 3).collect();
        assert!(r.is_empty());
    }

    #[test]
    fn test_range_with_map() {
        let r: Vec<i32> = Range::new(1, 4).map(|x| x * 2).collect();
        assert_eq!(r, vec![2, 4, 6]);
    }

    #[test]
    fn test_range_sum() {
        let sum: i32 = Range::new(1, 11).sum();
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_fibonacci_basic() {
        let fib: Vec<u64> = Fibonacci::new(7).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_fibonacci_zero() {
        let fib: Vec<u64> = Fibonacci::new(0).collect();
        assert!(fib.is_empty());
    }

    #[test]
    fn test_fibonacci_one() {
        let fib: Vec<u64> = Fibonacci::new(1).collect();
        assert_eq!(fib, vec![0]);
    }

    #[test]
    fn test_fibonacci_two() {
        let fib: Vec<u64> = Fibonacci::new(2).collect();
        assert_eq!(fib, vec![0, 1]);
    }

    #[test]
    fn test_fibonacci_large() {
        let fib: Vec<u64> = Fibonacci::new(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_sum() {
        let sum: u64 = Fibonacci::new(10).sum();
        assert_eq!(sum, 88);
    }

    #[test]
    fn test_cycle_basic() {
        let mut cycle = Cycle::new(&[1, 2, 3]);
        assert_eq!(cycle.next(), Some(&1));
        assert_eq!(cycle.next(), Some(&2));
        assert_eq!(cycle.next(), Some(&3));
        assert_eq!(cycle.next(), Some(&1));
        assert_eq!(cycle.next(), Some(&2));
    }

    #[test]
    fn test_cycle_single() {
        let mut cycle = Cycle::new(&[42]);
        assert_eq!(cycle.next(), Some(&42));
        assert_eq!(cycle.next(), Some(&42));
        assert_eq!(cycle.next(), Some(&42));
    }

    #[test]
    fn test_cycle_empty() {
        let empty: &[i32] = &[];
        let mut cycle = Cycle::new(empty);
        assert_eq!(cycle.next(), None);
        assert_eq!(cycle.next(), None);
    }

    #[test]
    fn test_cycle_with_take() {
        let items: Vec<&i32> = Cycle::new(&[1, 2]).take(5).collect();
        assert_eq!(items, vec![&1, &2, &1, &2, &1]);
    }

    #[test]
    fn test_cycle_strings() {
        let mut cycle = Cycle::new(&["a", "b"]);
        assert_eq!(cycle.next(), Some(&"a"));
        assert_eq!(cycle.next(), Some(&"b"));
        assert_eq!(cycle.next(), Some(&"a"));
    }
}
