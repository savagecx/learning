# Collecting Into Different Types

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, converting between collection types is effortless:

```python
numbers = [3, 1, 2, 1, 3]
unique = set(numbers)          # {1, 2, 3}
sorted_list = sorted(unique)   # [1, 2, 3]

pairs = [("a", 1), ("b", 2)]
lookup = dict(pairs)           # {"a": 1, "b": 2}

from collections import Counter
Counter(["red", "blue", "red"])  # {'red': 2, 'blue': 1}
```

You call `set()`, `dict()`, `list()`, `sorted()` — the constructor does the work. In Rust, one method does all of this: `.collect()`. What it collects *into* depends on the type annotation.

### `.collect()` — one method, many targets

`.collect()` consumes an iterator and builds a collection. The **type annotation** tells Rust which collection to create:

```rust
let v: Vec = (1..=5).collect();           // [1, 2, 3, 4, 5]

use std::collections::HashSet;
let set: HashSet = (1..=5).collect();     // {1, 2, 3, 4, 5}

let s: String = vec!['h', 'i'].into_iter().collect();  // "hi"
```

Same iterator, same `.collect()` call — different results. This is type-directed dispatch: Rust looks at the expected type and calls the right implementation.
You can also use **turbofish** syntax instead of a type annotation:

```rust
let v = (1..=5).collect::>();
```

Both approaches are equivalent. Use whichever reads better in context.

### Collecting into HashMap

An iterator of **tuples `(K, V)`** can be collected into a `HashMap`:

```rust
use std::collections::HashMap;

let map: HashMap<&str;, i32> = vec![("a", 1), ("b", 2)]
    .into_iter()
    .collect();
```

This is the Rust equivalent of `dict(pairs)` in Python. The iterator must yield 2-element tuples — the first becomes the key, the second the value.

### Collecting into HashSet for deduplication

`HashSet` automatically drops duplicates, just like Python's `set()`:

```rust
use std::collections::HashSet;
let unique: HashSet = vec![1, 2, 2, 3, 3, 3]
    .into_iter()
    .collect();
// {1, 2, 3}
```

Since `HashSet` is unordered (like Python's `set`), you'll often collect to a `HashSet`, then convert to a sorted `Vec` if order matters.

### The `FromIterator` trait

What makes `.collect()` so flexible is the `FromIterator` trait. Any type that implements `FromIterator<T>` can be a `.collect()` target. `Vec`, `HashSet`, `HashMap`, `String`, `BTreeMap` — they all implement it. You can even implement it for your own types.

### Counting with the entry API

Rust doesn't have a `Counter` equivalent in the standard library. Instead, you use the **entry API** on `HashMap` to build frequency counts:

```rust
let mut map = HashMap::new();
let value = map.entry("key").or_insert(0);  // inserts 0 if new, returns &mut; V
*value += 1;  // modify through the mutable reference
```

`.entry(key)` returns an entry that's either occupied or vacant. `.or_insert(default)` inserts the default if the key is new and returns a mutable reference to the value. Combine this with a loop over your data to build frequency counts — Rust's equivalent of Python's `Counter` or `defaultdict(int)`.

### Trait bounds: `Hash + Eq`

For a type to be used as a `HashMap` key or `HashSet` element, it must implement `Hash` (for hashing) and `Eq` (for equality). In Python, objects need `__hash__` and `__eq__` — same concept, different syntax. Rust's built-in types (`i32`, `String`, `&str`, etc.) all implement both.
When you write generic functions that work with hash-based collections, you'll specify these as **trait bounds**:

```rust
fn do_something(key: K) { /* ... */ }
```

This tells the compiler: "`K` can be any type, as long as it's hashable and comparable."

---


## Your Task


1. `group_by_first_char(words: &[&str])` — group words by their first character, returning `HashMap<char, Vec<String>>` (skip empty strings)
2. `char_frequencies(s: &str)` — count how many times each character appears, returning `HashMap<char, usize>`
3. `invert_map(map: HashMap<K, V>)` — swap keys and values, returning `HashMap<V, K>`


---


## Example


```rust
let groups = group_by_first_char(&["apple", "avocado", "banana"]);
assert_eq!(groups.get(&'b').unwrap(), &vec;!["banana".to_string()]);

let freq = char_frequencies("hello");
assert_eq!(freq.get(&'l'), Some(&2));

let map: HashMap<&str;, i32> = [("a", 1), ("b", 2)].into_iter().collect();
let inverted = invert_map(map);
assert_eq!(inverted.get(&1), Some(&"a"));
```


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — collecting into different types
- [Iterator::collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) — how type annotations direct collection
- [FromIterator trait](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) — implement collect for your own types
- [HashMap entry API](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) — the pattern for counting and grouping

