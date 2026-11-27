struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn into_inner(self) -> T {
        self.value
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_new_int() {
        let w = Wrapper::new(42);
        assert_eq!(w.into_inner(), 42);
    }

    #[test]
    fn test_wrapper_new_string() {
        let w = Wrapper::new(String::from("hello"));
        assert_eq!(w.into_inner(), "hello");
    }

    #[test]
    fn test_wrapper_new_vec() {
        let w = Wrapper::new(vec![1, 2, 3]);
        assert_eq!(w.into_inner(), vec![1, 2, 3]);
    }

    #[test]
    fn test_pair_new_same_types() {
        let p = Pair::new(1, 2);
        assert_eq!(p.first, 1);
        assert_eq!(p.second, 2);
    }

    #[test]
    fn test_pair_new_different_types() {
        let p = Pair::new("hello", 42);
        assert_eq!(p.first, "hello");
        assert_eq!(p.second, 42);
    }

    #[test]
    fn test_pair_swap_same_types() {
        let p = Pair::new(1, 2);
        let swapped = p.swap();
        assert_eq!(swapped.first, 2);
        assert_eq!(swapped.second, 1);
    }

    #[test]
    fn test_pair_swap_different_types() {
        let p = Pair::new("hello", 123);
        let swapped = p.swap();
        assert_eq!(swapped.first, 123);
        assert_eq!(swapped.second, "hello");
    }

    #[test]
    fn test_pair_swap_twice() {
        let p = Pair::new(1, 2);
        let swapped = p.swap().swap();
        assert_eq!(swapped.first, 1);
        assert_eq!(swapped.second, 2);
    }

    #[test]
    fn test_wrapper_with_pair() {
        let p = Pair::new(1, 2);
        let w = Wrapper::new(p);
        let inner = w.into_inner();
        assert_eq!(inner.first, 1);
        assert_eq!(inner.second, 2);
    }
}
