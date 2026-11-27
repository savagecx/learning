# Simple Calculations

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, numbers "just work" — `42` is an `int`, `3.14` is a `float`, and you can mix them freely:

```python
result = 42 + 3.14  # fine, Python promotes int to float
```

Rust is stricter. It has **many** numeric types, and it won't convert between them automatically:

```rust
let x: i32 = 42;
let y: f64 = 3.14;
// let result = x + y;  // ERROR: can't add i32 and f64
let result = x as f64 + y;  // OK: explicit cast with `as`
```

Here are the most common ones:
| Rust | Python equivalent | What it is |
|------|-------------------|------------|
| `i32` | `int` | 32-bit signed integer (default integer type) |
| `u32` | `int` (positive only) | 32-bit *unsigned* integer (no negatives) |
| `f64` | `float` | 64-bit floating point (default float type) |
| `bool` | `bool` | `true`/`false` (lowercase in Rust!) |
Why so many types? Performance and correctness. When Rust knows a value is a `u32`, it knows it fits in 4 bytes and is never negative — the compiler can optimize accordingly and catch mistakes at compile time.

### Expression-based returns

In Rust, the last expression in a function body *is* the return value — no `return` keyword needed:

```rust
fn double(x: i32) -> i32 {
    x * 2  // no semicolon = this is the return value
}
```

Adding a semicolon turns it into a *statement* (which returns `()`, Rust's "nothing" type), so be careful:

```rust
fn double(x: i32) -> i32 {
    x * 2;  // semicolon makes this return () — compile error!
}
```

This is one of those small things that trips up every newcomer. If you get an error about "expected `i32`, found `()`", check for an accidental semicolon.

---


## Your Task

Implement two functions:

1. `celsius_to_fahrenheit(celsius: f64) -> f64` — convert Celsius to Fahrenheit using the formula: `F = C * 9/5 + 32`
2. `is_even(n: u32) -> bool` — return `true` if `n` is even, `false` otherwise


---


## Example


```rust
assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
assert_eq!(is_even(2), true);
assert_eq!(is_even(3), false);
```


---


## Further Reading


- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) — all the numeric types and when to use them
- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — function signatures, parameters, and return values

