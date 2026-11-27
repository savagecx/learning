# Functions and Return Values

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, functions use `def`, type hints are optional, and you return with `return`:

```python
def area(width: int, height: int) -> int:
    return width * height
```

Rust functions look similar but with important differences:

```rust
fn area(width: i32, height: i32) -> i32 {
    width * height
}
```

Three things to notice:

1. **Types are required** — every parameter and the return type must be declared. No optional hints.
2. **Expression-based returns** — the last expression (without a semicolon) is the return value. No `return` keyword needed.
3. **Semicolons matter** — adding a semicolon to the last line turns it into a statement and the function returns `()` (Rust's unit type, like Python's `None`).


### The semicolon trap

This is a common gotcha for Python developers:

```rust
fn broken(width: i32, height: i32) -> i32 {
    width * height;  // semicolon makes this a statement, not a return value
    // ERROR: expected i32, found ()
}
```

Remove the semicolon and it works. The `return` keyword exists but is mainly used for early returns from within branches.

---


## Your Task

Implement `add_and_double` — takes two `i32` values, returns `(a + b) * 2`.
Use expression-based return (no `return` keyword).

---


## Example


```rust
assert_eq!(add_and_double(2, 3), 10);
assert_eq!(add_and_double(-1, 4), 6);
```


---


## Further Reading


- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — parameters, return types, and expressions
- [The Rust Book — Statements and Expressions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions) — why semicolons matter

