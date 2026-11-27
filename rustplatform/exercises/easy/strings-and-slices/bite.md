# Strings and Slices

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, strings are sequences and slicing is simple:

```python
s = "hello world"
first_word = s.split()[0]      # "hello"
last_three = s[-3:]            # "rld"
```

Rust strings work differently because of its ownership model. There are two main string types:

- **`&str`** — a string slice. A borrowed, read-only view into string data. This is what string literals (`"hello"`) give you.
- **`String`** — an owned, heap-allocated string. Like Python's `str`, but you're responsible for when it's created and dropped.


### Slicing strings

Rust slices use byte indices with range syntax:

```rust
let s = "hello world";
let first_five = &s;[0..5];   // "hello"
let from_six = &s;[6..];      // "world"
```

This is similar to Python's `s[0:5]`, but Rust uses `..` instead of `:`. The ranges are half-open (end is excluded), just like Python.
Important: Rust string slices are byte-indexed, not character-indexed. For ASCII strings this doesn't matter. For Unicode, slicing at the wrong byte offset causes a panic. We'll keep inputs ASCII in this exercise.

### Returning slices

When a function takes `&str` and returns `&str`, it returns a **view** into the input — no allocation, no copy:

```rust
fn get_domain(url: &str;) -> &str; {
    // returns a slice of the input string
}
```

This is like Python's `memoryview` concept — a zero-cost window into existing data. The Rust compiler ensures the returned slice doesn't outlive the input.

---


## Your Task

Implement two functions:

1. `first_word(s: &str) -> &str` — return the substring up to the first space. If there's no space, return the whole string.
2. `last_n(s: &str, n: usize) -> &str` — return the last `n` characters. If `n` exceeds the string length, return the whole string. (Assume ASCII input.)


---


## Example


```rust
assert_eq!(first_word("hello world"), "hello");
assert_eq!(first_word("rustacean"), "rustacean");

assert_eq!(last_n("abcdef", 3), "def");
assert_eq!(last_n("hi", 5), "hi");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), your tokenizer will slice into the JSON input string to extract tokens — string values, number literals, keywords — all as `&str` views with zero copying.


---


## Further Reading


- [The Rust Book — The String Type](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type) — String vs &str
- [The Rust Book — String Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices) — slicing syntax and safety

