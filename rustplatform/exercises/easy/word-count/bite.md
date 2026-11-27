# Word Count

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, counting words is a one-liner with `collections.Counter`:

```python
from collections import Counter
# simplified — doesn't strip punctuation
counts = Counter(s.lower().split())
```

In Rust, there's no `Counter`, but there's `HashMap` — and learning how to build one up is a fundamental Rust skill.

### HashMap — Rust's `dict`

`HashMap<K, V>` is Rust's equivalent of Python's `dict`. Unlike Python where `dict` is a builtin, you need to import it:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
```

Notice the `mut` — in Rust, variable bindings are immutable by default, so you need `mut` on `scores` to insert, remove, or update entries in the map.

### The entry API — Rust's `setdefault` on steroids

In Python, you might count occurrences like this:

```python
counts = {}
for n in numbers:
    counts[n] = counts.get(n, 0) + 1
```

Or with `Counter(numbers)`. Rust has no `Counter`, but it has an elegant equivalent — the **entry API**:

```rust
let mut frequencies = HashMap::new();
for n in [1, 2, 2, 3, 3, 3] {
    *frequencies.entry(n).or_insert(0) += 1;
}
// {1: 1, 2: 2, 3: 3}
```

Let's break this down:
- `.entry(key)` looks up the key and returns an `Entry` enum — either `Occupied` or `Vacant`
- `.or_insert(0)` inserts `0` if the key is missing, then returns a `&mut V` (mutable reference to the value)
- `*` dereferences that mutable reference so you can modify the actual value
- `+= 1` increments it
This pattern is idiomatic Rust for counting and accumulating into HashMaps.

### Splitting strings

Rust has `.split_whitespace()` which works like Python's `.split()` (no arguments) — it splits on any whitespace and skips empty chunks:

```rust
let words: Vec<&str;> = "hello   world".split_whitespace().collect();
// ["hello", "world"]
```


---


## Your Task

Implement `word_count(s: &str) -> HashMap<String, usize>` that counts the occurrences of each word in a string. Words should be lowercased and stripped of punctuation.

---


## Example


```rust
let counts = word_count("Hello hello world");
assert_eq!(counts.get("hello"), Some(&2));
assert_eq!(counts.get("world"), Some(&1));
```


---


## Further Reading


- [The Rust Book — Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) — creating, updating, and the entry API
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) — full API reference
- [HashMap::entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) — the entry API in detail

