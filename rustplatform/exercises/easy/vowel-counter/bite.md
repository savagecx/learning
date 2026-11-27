# Vowel Counter

- Level: easy
- Author: ToHellAndBeck

## Instructions
🎯 In Python, you can iterate directly over characters in a string: `for c in "hello"`. You can also use a generator expression like `sum(1 for c in s if c.isdigit())`.
In Rust, strings are **UTF-8 encoded bytes**, not arrays of characters. You can't index into them with `s[0]` like in Python — that would be a byte offset, not necessarily a character. Instead, you use **`.chars()`** to get an iterator over Unicode characters:

```rust
for c in "hello".chars() {
    println!("{c}");
}
```

This iterator pattern is central to Rust. Instead of building up a count variable in a loop, Rust encourages **chaining iterator methods** — similar to how you'd chain operations in a Python generator expression:

```rust
let digits = "abc123".chars()
    .filter(|c| c.is_ascii_digit())
    .count();
// 3
```

Notice the **closure** syntax: `|c|` defines an inline function (like Python's `lambda c: ...`). The `.filter()` method keeps only elements matching the condition, and `.count()` consumes the iterator and returns how many items passed through.

---


## Your Task

Implement `count_vowels(s: &str) -> usize` that returns the number of vowels (a, e, i, o, u — both lowercase and uppercase) in the given string.
The function takes a `&str` (a borrowed string slice) — meaning it *reads* the string without taking ownership. The caller keeps their string.

---


## Example


```rust
assert_eq!(count_vowels("hello"), 2);
assert_eq!(count_vowels("RUST"), 1);
assert_eq!(count_vowels("bcdfgh"), 0);
```


---


## Further Reading


- [The Rust Book — Strings](https://doc.rust-lang.org/book/ch08-02-strings.html) — why you can't index Rust strings and how `.chars()` works
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) — the `|x|` syntax used in `.filter()` and other iterator methods
- [std::str::Chars](https://doc.rust-lang.org/std/str/struct.Chars.html) — the iterator returned by `.chars()`

