# Reverse a String

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, reversing a string is a one-liner: `s[::-1]`. Rust doesn't have slicing syntax like that, but it has something arguably more powerful: **iterator adapters**.
You already know `.chars()` gives you an iterator over characters. Rust iterators have a rich set of *adapter* methods that transform the sequence without consuming it:

```rust
let nums = vec![1, 2, 3, 4, 5];
let first_three: Vec = nums.iter()
    .take(3)
    .copied()
    .collect();
// [1, 2, 3]
```

Notice **`.collect()`** at the end — this is a key Rust concept. Iterators are *lazy*: adapter methods like `.take()`, `.map()`, and `.filter()` just build up a chain of transformations. Nothing actually happens until you *consume* the iterator. `.collect()` is the most common consumer — it gathers results into a collection.
The magic: `.collect()` figures out *what type* to build from the return type. It can produce a `Vec<T>`, a `String`, a `HashSet<T>`, and more — all from the same method call. The compiler uses the expected type to decide:

```rust
let v: Vec = "hello".chars().collect();    // Vec of chars
let s: String = "hello".chars().collect();        // back to a String
```

**Explore the iterator docs** — there are adapters for skipping, filtering, transforming, reversing, and much more. Browsing the [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) methods is a great way to discover what's available.

---


## Your Task

Implement `reverse_string(input: &str) -> String` that returns a new `String` with the characters in reverse order.
The function borrows the input (`&str`) and returns a new owned `String` — the original is untouched.

---


## Example


```rust
assert_eq!(reverse_string("hello"), "olleh");
assert_eq!(reverse_string("rust"), "tsur");
```


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — how iterator adapters and consumers work
- [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — the full list of adapter methods (`.rev()`, `.take()`, `.map()`, etc.)
- [std::iter::Iterator::collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) — how type inference drives collection

