# Hello, Rustacean!

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, returning a string from a function is straightforward:

```python
def motto() -> str:
    return "Fearless concurrency"
```

Rust works similarly, but with a few differences you'll see right away:

```rust
fn motto() -> String {
    "Fearless concurrency".to_string()
}
```

The `fn` keyword replaces `def`. The return type goes after `->` (just like Python type hints, but required in Rust). And the last expression without a semicolon is automatically returned — no `return` keyword needed.

### String vs &str — a first look

You'll notice the return type is `String`, not `str`. Rust has two main string types:

- `&str` — a string slice (borrowed, like a read-only view)
- `String` — an owned string (heap-allocated, like Python's `str`)

For now, just know that `"Hello"` is a `&str` literal, and `.to_string()` or `.into()` converts it to an owned `String`. We'll explore this distinction in depth later.

---


## Your Task

Implement the `greet` function to return `"Hello, Rustacean!"` as an owned `String`.

---


## Example


```rust
assert_eq!(greet(), "Hello, Rustacean!");
```


---


## Further Reading


- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — defining functions and return values
- [The Rust Book — The String Type](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type) — String vs &str introduction

