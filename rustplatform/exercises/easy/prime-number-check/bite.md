# Prime Number Check

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you might write a prime check like this:

```python
def is_perfect_square(n):
    if n < 0:
        return False
    root = int(n**0.5)
    return root * root == n
```

Checking primality in Rust follows similar logic to what you'd write in Python, but introduces a few language features worth understanding.

### Early return

So far you've seen Rust functions where the last expression is the return value. But sometimes you want to bail out early — that's what `return` is for:

```rust
fn check(n: u64) -> bool {
    if n == 0 {
        return false;  // exit immediately
    }
    // ... more logic ...
    true  // last expression = default return
}
```

Both styles are idiomatic. Use expression-based returns when the function flows naturally to a single result. Use `return` when you need to short-circuit — like rejecting invalid input before the main logic runs.

### Type casting with `as`

To compute a square root, you need a float — but the input is `u64`. Rust won't convert automatically, so you cast with `as`:

```rust
let bytes: u64 = 1_500_000;
let megabytes = bytes as f64 / 1_000_000.0;  // 1.5
let rounded = megabytes as u64;               // 1
```

`as` is Rust's low-level cast. It works between numeric types, but be aware: casting a large `u64` to `f64` can lose precision (floats only have 53 bits of mantissa). For this exercise that's fine, but in general, prefer `.try_into()` when precision matters.

### `.step_by()` — range with a step

Python's `range()` takes a step argument: `range(3, 100, 2)`. In Rust, you chain `.step_by()` onto a range:

```rust
for i in (3..100).step_by(2) {
    println!("{i}");  // 3, 5, 7, 9, ...
}
```

This is useful for skipping even numbers when checking divisors.

---


## Your Task

Implement `is_prime(n: u64) -> bool` that returns `true` if `n` is a prime number.
A prime number is a natural number greater than 1 with no positive divisors other than 1 and itself.

---


## Example


```rust
assert!(is_prime(2));
assert!(is_prime(97));
assert!(!is_prime(1));
assert!(!is_prime(4));
```


---


## Further Reading


- [The Rust Book — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — `if`, loops, and early returns
- [The Rust Reference — Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions) — how `as` works between types
- [Iterator::step_by](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by) — stepping through ranges

