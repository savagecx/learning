# URL Query Parameter Parser

- Level: medium
- Author: bbelderbos

## Instructions
🎯 In Python, parsing structured strings into dicts is straightforward:

```python
# colon-delimited key:value pairs
pairs = "name:Alice,age:30".split(",")
data = dict(p.split(":", 1) for p in pairs)
```

This exercise applies the same idea to URL query strings in Rust — a great chance to practice combining iterator methods into a real parsing pipeline.

### String splitting

Rust's `.split()` works like Python's, returning an iterator over the pieces:

```rust
let parts: Vec<&str;> = "a&b;&c;".split('&').collect();
// ["a", "b", "c"]
```

When you need to **limit** the number of splits, use `.splitn(n, pattern)` — like Python's `str.split(':', 1)`:

```rust
let parts: Vec<&str;> = "host:localhost:8080".splitn(2, ':').collect();
// ["host", "localhost:8080"]  — only splits on the first ':'
```

This matters when the value itself may contain the delimiter — you want to split into key and rest, not all pieces.

### Multi-line closures

So far you've seen short closures like `|x| x + 1`. When you need multiple statements, use curly braces:

```rust
let results: Vec = items.iter().map(|item| {
    let doubled = item * 2;
    let adjusted = doubled + 1;
    adjusted
}).collect();
```

The last expression in the block is the closure's return value — same rule as functions.

### Collecting tuples into a HashMap

You already know `.collect()` can build a `Vec` or `String`. It can also build a `HashMap` — if the iterator yields *tuples* of `(key, value)`:

```rust
use std::collections::HashMap;

let pairs = vec![("name", "Alice"), ("age", "30")];
let map: HashMap<&str;, &str;> = pairs.into_iter().collect();
```

This is the Rust-idiomatic way to build a HashMap from parsed data — transform your input into an iterator of tuples, then `.collect()`.

### Handling missing values with `.unwrap_or()`

When pulling items from an iterator with `.next()`, the result is `Option<T>` — it might be `None` if the iterator is exhausted. `.unwrap_or(default)` provides a fallback:

```rust
let mut parts = "Alice:42".splitn(2, ':');
let name = parts.next().unwrap_or("unknown");  // "Alice"
let age = parts.next().unwrap_or("0");          // "42"
```


---


## Your Task

Implement `parse_query_params(query: &str) -> HashMap<String, String>` that parses a URL query string like `"name=John&age=30"` into a HashMap of key-value pairs.
Handle edge cases: empty strings and parameters without values (e.g., `"name="` → `("name", "")`).

---


## Example


```rust
let params = parse_query_params("name=John&age;=30");
assert_eq!(params.get("name"), Some(&"John".to_string()));
assert_eq!(params.get("age"), Some(&"30".to_string()));
```


---


## Further Reading


- [str::split](https://doc.rust-lang.org/std/primitive.str.html#method.split) — splitting strings by pattern
- [str::splitn](https://doc.rust-lang.org/std/primitive.str.html#method.splitn) — splitting with a limit
- [Option::unwrap_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) — providing defaults for Option values

