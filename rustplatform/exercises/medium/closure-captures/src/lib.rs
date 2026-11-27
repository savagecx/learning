fn counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn create_greeting(name: String) -> impl Fn() -> String {
    move || format!("Hello, {name}!")
}

fn accumulator(initial: i32) -> impl FnMut(i32) -> i32 {
    let mut count = initial;
    move |inc| {
        count += inc;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increments() {
        let mut count = counter();
        assert_eq!(count(), 1);
        assert_eq!(count(), 2);
        assert_eq!(count(), 3);
    }

    #[test]
    fn test_counter_independent() {
        let mut count1 = counter();
        let mut count2 = counter();
        assert_eq!(count1(), 1);
        assert_eq!(count1(), 2);
        assert_eq!(count2(), 1);
        assert_eq!(count1(), 3);
        assert_eq!(count2(), 2);
    }

    #[test]
    fn test_counter_many_calls() {
        let mut count = counter();
        for i in 1..=100 {
            assert_eq!(count(), i);
        }
    }

    #[test]
    fn test_create_greeting_basic() {
        let greet = create_greeting(String::from("Alice"));
        assert_eq!(greet(), "Hello, Alice!");
    }

    #[test]
    fn test_create_greeting_multiple_calls() {
        let greet = create_greeting(String::from("Bob"));
        assert_eq!(greet(), "Hello, Bob!");
        assert_eq!(greet(), "Hello, Bob!");
    }

    #[test]
    fn test_create_greeting_different_names() {
        let greet_alice = create_greeting(String::from("Alice"));
        let greet_bob = create_greeting(String::from("Bob"));
        assert_eq!(greet_alice(), "Hello, Alice!");
        assert_eq!(greet_bob(), "Hello, Bob!");
    }

    #[test]
    fn test_accumulator_basic() {
        let mut acc = accumulator(10);
        assert_eq!(acc(5), 15);
        assert_eq!(acc(3), 18);
    }

    #[test]
    fn test_accumulator_from_zero() {
        let mut acc = accumulator(0);
        assert_eq!(acc(1), 1);
        assert_eq!(acc(2), 3);
        assert_eq!(acc(3), 6);
    }

    #[test]
    fn test_accumulator_negative() {
        let mut acc = accumulator(100);
        assert_eq!(acc(-10), 90);
        assert_eq!(acc(-20), 70);
    }

    #[test]
    fn test_accumulator_independent() {
        let mut acc1 = accumulator(0);
        let mut acc2 = accumulator(100);
        assert_eq!(acc1(5), 5);
        assert_eq!(acc2(5), 105);
        assert_eq!(acc1(5), 10);
        assert_eq!(acc2(5), 110);
    }
}
