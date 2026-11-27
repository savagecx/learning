# Palindrome Check

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you might check a palindrome like this:

```python
cleaned = "".join(c.lower() for c in s if c.isalnum())
return cleaned == cleaned[::-1]
```

This exercise is about doing the same thing in Rust — but it's a great chance to practice **composing iterator chains**, combining the tools you've picked up in the previous exercises.

### `char` methods you'll find useful

Rust's `char` type has built-in methods that mirror Python's `str` methods, but they work on *individual characters*:

```rust
'A'.is_alphanumeric()   // true  (like Python's str.isalnum())
'!'.is_alphanumeric()   // false
'A'.to_lowercase()      // an iterator yielding 'a'
```

One subtlety: `char::to_lowercase()` returns an **iterator**, not a single `char`. Why? Because some Unicode characters lowercase into multiple characters (e.g., German ß). This means you can't just `.map()` it into a single char — you'll need **`.flat_map()`** to flatten those inner iterators into one stream:

```rust
let letters: String = "Hello!"
    .chars()
    .flat_map(|c| c.to_lowercase())
    .collect();
// "hello!"
```

`.flat_map()` is like `.map()` followed by `.flatten()` — if your closure returns an iterator (or `Option`, or `Vec`), it unpacks the results into a single flat sequence. Python's equivalent is roughly a nested comprehension: `[x for group in items for x in group]`.

### The turbofish `::<Type>`

Sometimes `.collect()` can't infer the target type from context. You can specify it inline with **turbofish syntax**:

```rust
let s = some_iterator.collect::();
```

This is equivalent to `let s: String = some_iterator.collect();` — just a different place to put the type annotation.

---


## Your Task

Implement `is_palindrome(s: &str) -> bool` that checks whether a string is a palindrome, ignoring case and non-alphanumeric characters.

---


## Example


```rust
assert!(is_palindrome("racecar"));
assert!(is_palindrome("A man a plan a canal Panama"));
assert!(!is_palindrome("hello"));
```


---


## Further Reading


- [std::char — Character methods](https://doc.rust-lang.org/std/primitive.char.html) — `is_alphanumeric()`, `to_lowercase()`, and more
- [Iterator::flat_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) — flattening nested iterators
- [The Rust Book — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — composing adapter chains

