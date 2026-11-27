use std::ops::Div;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let root = (n as f64).sqrt();

    if root.fract() == 0_f64 {
        return false;
    }

    for i in 1..=(root as u64) {
        if is_prime(i) && (n as f64).div(i as f64).fract() == 0_f64 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
    }

    #[test]
    fn test_not_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
    }

    #[test]
    fn test_larger_primes() {
        assert!(is_prime(97));
        assert!(is_prime(101));
        assert!(is_prime(103));
        assert!(is_prime(997));
    }

    #[test]
    fn test_larger_composites() {
        assert!(!is_prime(100));
        assert!(!is_prime(121)); // 11 * 11
        assert!(!is_prime(1000));
    }
}
