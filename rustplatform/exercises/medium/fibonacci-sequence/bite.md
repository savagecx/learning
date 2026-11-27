# Fibonacci Sequence

- Level: medium
- Author: Pybites

## Instructions
🎯 Python 3.10 introduced structural pattern matching with `match`/`case`. Rust has had `match` from the start — and it's one of the language's most powerful features.

### `match` in Rust

Where Python uses `if`/`elif` chains or `match`/`case`, Rust uses `match` expressions:

```rust
match n {
    0 => println!("zero"),
    1 => println!("one"),
    _ => println!("something else"),  // _ is the catch-all (like Python's case _)
}
```

Two things make Rust's `match` stand out:

1. **It's exhaustive** — you *must* handle every possible case. Forget a variant? Compiler error. This prevents bugs that `if`/`elif` chains silently allow.
2. **It's an expression** — `match` returns a value, so you can use it on the right side of `let` or as a function's return:


```rust
let label = match n {
    0 => "zero",
    1 => "one",
    _ => "other",
};
```


### Match guards

You can add conditions to match arms with `if`:

```rust
match n {
    x if x < 0 => println!("negative"),
    0 => println!("zero"),
    _ => println!("positive"),
}
```

The `x if x < 0` is a **match guard** — it binds the value to `x` and only matches if the condition is true.

### `panic!` — Rust's "raise Exception"

When something goes truly wrong, Rust has `panic!`:

```rust
panic!("something went horribly wrong: {}", details);
```

This immediately terminates the current thread with an error message — similar to Python's `raise Exception(...)`. In production Rust code, you'd usually prefer returning a `Result` (we'll cover that in later exercises), but `panic!` is appropriate when a function receives input that should *never* happen.
In tests, you can assert that code panics:

```rust
#[test]
#[should_panic]
fn test_negative_input() {
    my_function(-1);  // this test passes if my_function panics
}
```


---


## Your Task

Implement `fibonacci(n: i32) -> u32` that returns the nth Fibonacci number.
The sequence starts: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...

- `fibonacci(0)` returns `0`
- `fibonacci(1)` returns `1`
- For negative input, panic with `"Negative input is not allowed"`

Notice the signature: the input is `i32` (signed, to accept negative values) but the output is `u32` (unsigned, because Fibonacci numbers are never negative).

---


## Example


```rust
assert_eq!(fibonacci(0), 0);
assert_eq!(fibonacci(5), 5);
assert_eq!(fibonacci(10), 55);
```


---


## Further Reading


- [The Rust Book — The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html) — exhaustive matching, guards, and the `_` placeholder
- [The Rust Book — Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html) — when and how to use `panic!`

