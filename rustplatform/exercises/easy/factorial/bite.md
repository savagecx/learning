# Factorial

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you never worry about integer size — `math.factorial(100)` returns a 158-digit number without breaking a sweat. Python integers grow as large as memory allows.
Rust integers have **fixed sizes**. A `u64` maxes out at 18,446,744,073,709,551,615. That sounds huge, but `21!` already exceeds it — factorials grow *fast*. In debug mode, Rust **panics on integer overflow** rather than silently wrapping around. This is a safety feature: it catches bugs that would go unnoticed in C.
The function signature uses `u64` — enough to handle up to `factorial(20)`.

### Or-patterns in match

In the previous exercise you used `match` with individual arms. You can also combine multiple patterns with `|` (or):

```rust
match day {
    0 | 6 => println!("weekend"),  // matches 0 OR 6
    _ => println!("weekday"),
}
```

This is cleaner than writing two separate arms with the same body. Python's equivalent is `case 0 | 6:`.

### Ranges — Rust's `range()`

If you go the iterative route, you'll want Rust's **range** syntax:

```rust
for i in 1..5 {
    print!("{i} ");  // 1 2 3 4 (exclusive end, like Python's range)
}

for i in 1..=5 {
    print!("{i} ");  // 1 2 3 4 5 (inclusive end)
}
```

`1..5` is like Python's `range(1, 5)`. `1..=5` includes the end — there's no direct Python equivalent, you'd write `range(1, 6)`.
Ranges are iterators, so all the iterator methods work on them — `.map()`, `.filter()`, `.fold()`, and more.

---


## Your Task

Implement `factorial(n: u64) -> u64` that returns n! (the factorial of n).

- `0! = 1` and `1! = 1` (by definition)
- For n > 1, n! = n × (n-1) × ... × 2 × 1

You can solve this recursively or iteratively — both approaches work.

---


## Example


```rust
assert_eq!(factorial(0), 1);
assert_eq!(factorial(5), 120);
assert_eq!(factorial(10), 3628800);
```


---


## Further Reading


- [The Rust Book — The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html) — or-patterns and combining arms
- [std::ops::Range](https://doc.rust-lang.org/std/ops/struct.Range.html) — Rust's range types and how they work as iterators
- [Integer overflow behavior](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) — what happens when numbers get too big

