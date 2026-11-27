# HashMap Entry API

- Level: medium
- Author: Pybites

## Instructions
In Python, dictionaries handle missing keys gracefully:

```python
counts = {}
counts['a'] = counts.get('a', 0) + 1  # increment or initialize

# Or with defaultdict
from collections import defaultdict
counts = defaultdict(int)
counts['a'] += 1
```

Rust's `HashMap` has an **entry API** that's even more powerful.

### Basic HashMap operations


```rust
use std::collections::HashMap;

let mut map: HashMap = HashMap::new();
map.insert("key".to_string(), 42);

let value = map.get("key");         // Option<&i32;>
let value = map["key"];             // panics if missing
```


### The Entry API

The entry API lets you inspect and modify an entry in one step:

```rust
use std::collections::hash_map::Entry;

// Insert if missing, get mutable reference either way
let count = map.entry("key".to_string()).or_insert(0);
*count += 1;

// Insert with a closure (lazy computation)
map.entry("key".to_string()).or_insert_with(|| expensive_default());

// Modify existing entry
map.entry("key".to_string()).and_modify(|v| *v += 1);

// Combine: modify if exists, insert if missing
map.entry("key".to_string())
    .and_modify(|v| *v += 1)
    .or_insert(1);
```


### Counting pattern


```rust
let rolls = [3, 6, 3, 2, 6, 3];
let mut frequencies: HashMap = HashMap::new();

for roll in rolls {
    *frequencies.entry(roll).or_insert(0) += 1;
}
// {3: 3, 6: 2, 2: 1}
```

This is Rust's equivalent of Python's `Counter` or `defaultdict(int)`.

---


## Your Task


1. `word_count(text: &str) -> HashMap<String, i32>` — count word frequencies
2. `group_by_length(words: Vec<String>) -> HashMap<usize, Vec<String>>` — group words by their length
3. `merge_maps(a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32>` — merge two maps, summing values for duplicate keys


---


## Example


```rust
let counts = word_count("the cat and the dog");
assert_eq!(counts.get("the"), Some(&2));
assert_eq!(counts.get("cat"), Some(&1));

let grouped = group_by_length(vec!["a".into(), "bb".into(), "ccc".into(), "dd".into()]);
assert_eq!(grouped.get(&1), Some(&vec;!["a".to_string()]));
assert_eq!(grouped.get(&2), Some(&vec;!["bb".to_string(), "dd".to_string()]));

let a = HashMap::from([("x".into(), 1), ("y".into(), 2)]);
let b = HashMap::from([("y".into(), 3), ("z".into(), 4)]);
let merged = merge_maps(a, b);
assert_eq!(merged.get("y"), Some(&5));  // 2 + 3
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), `HashMap<String, JsonValue>` is how you'll represent JSON objects — and the entry API helps you build them efficiently during parsing.


---


## Further Reading


- [The Rust Book — HashMaps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Entry API in std](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)

