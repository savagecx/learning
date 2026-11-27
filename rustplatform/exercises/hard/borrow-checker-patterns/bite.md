# Borrow Checker Patterns

- Level: hard
- Author: Pybites

## Instructions
🎯 In Python, you rarely think about data ownership when writing functions. You pass objects around, mutate lists in place, build new strings — the GC handles the rest:

```python
def format_path(parts):
    return "/".join(parts)

def rotate_left(items):
    items.append(items.pop(0))

text.upper()
```

In Rust, the borrow checker enforces rules about who can read and write data. At first, this feels like fighting the compiler. But experienced Rustaceans have a set of patterns that make borrow checker issues disappear. These patterns aren't workarounds — they produce clearer, safer code.

### Pattern 1: Return owned data instead of borrowing

When a function transforms input into something new, return an owned type rather than trying to borrow:

```rust
fn to_csv(values: &[i32]) -> String {
    values.iter().map(|v| v.to_string()).collect::>().join(",")
}
```

In Python, `",".join(str(v) for v in values)` returns a new string. The same principle applies in Rust — returning a `String` instead of `&str` avoids lifetime complexity. You trade a small allocation for a much simpler signature.
**Rule of thumb:** if the output is a transformation of the input (not a view into it), return owned data.

### Pattern 2: Use `.swap()` and `split_at_mut()` for in-place mutation

The borrow checker prevents two mutable references to the same data. So this won't compile:

```rust
let first = &mut; v[0];
let last = &mut; v[v.len() - 1];  // ERROR: can't borrow v again
std::mem::swap(first, last);
```

Rust provides methods that safely split mutable access:

```rust
v.rotate_left(1);  // moves first element to the end
```

Or for accessing two mutable elements simultaneously:

```rust
let (left, right) = v.split_at_mut(mid);
// left and right are non-overlapping — both safely mutable
```

In Python, `items.append(items.pop(0))` rotates easily. In Rust, look at methods on slices like `.swap()`, `.rotate_left()`, and `.split_at_mut()` — each safely handles in-place mutation.

### Pattern 3: Clone to break borrow chains

When the borrow checker complains about overlapping borrows, sometimes the simplest fix is to clone:

```rust
let name = data.name.clone();  // break the borrow on data
process(&mut; data, &name;);     // now data can be mutably borrowed
```

In Python, everything is reference-counted, so sharing is free. In Rust, `.clone()` has a cost — but it's explicit, and often the clearest solution. Don't optimize away clones until profiling shows they matter.

### Pattern 4: Owned wrappers for complex state

When you need to build up data over time — appending to a buffer, accumulating results — use an owned struct:

```rust
struct Logger {
    entries: Vec,
}

impl Logger {
    fn log(&mut; self, msg: &str;) {
        self.entries.push(msg.to_string());
    }
}
```

The struct owns its `Vec`, so there are no lifetime parameters. Methods take `&mut self` to mutate and `&self` to read. This is the Rust equivalent of a Python class with mutable state — the ownership is just explicit.

### Pattern 5: Process and return owned results

When you need multiple derived values from borrowed data, compute them all and return owned results:

```rust
fn analyze(data: &[i32]) -> (Vec, usize) {
    let positives: Vec = data.iter().filter(|&&x;| x > 0).copied().collect();
    let count = positives.len();
    (positives, count)
}
```

The input is borrowed (`&[i32]`), but both outputs are owned. No lifetime annotations needed. The function borrows input, processes it, and hands back self-contained results.

---


## Your Task


1. `join_words(words: &[&str]) -> String` — join words with spaces, returning an owned `String`
2. `swap_first_last(v: &mut [i32])` — swap first and last elements in place (handle short slices)
3. `find_and_replace(text, find, replace) -> String` — replace all occurrences, returning a new `String`
4. `extract_and_process(data: &[i32]) -> (Vec<i32>, i32)` — extract even numbers and return `(evens, sum_of_evens)`
5. `Buffer` struct — implement `new()`, `append()`, `get()`, and `to_string_lossy()` on an owned byte buffer


---


## Example


```rust
assert_eq!(join_words(&["hello", "world"]), "hello world");

let mut nums = vec![1, 2, 3, 4, 5];
swap_first_last(&mut; nums);
assert_eq!(nums, vec![5, 2, 3, 4, 1]);

assert_eq!(find_and_replace("hello world", "world", "rust"), "hello rust");

let (evens, sum) = extract_and_process(&[1, 2, 3, 4, 5, 6]);
assert_eq!(evens, vec![2, 4, 6]);
assert_eq!(sum, 12);

let mut buf = Buffer::new();
buf.append(b"hello");
assert_eq!(buf.get(0), Some(b'h'));
assert_eq!(buf.to_string_lossy(), "hello");
```


---


## Further Reading


- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — the borrowing rules
- [slice::swap](https://doc.rust-lang.org/std/primitive.slice.html#method.swap) — safe in-place element swapping
- [slice::split_at_mut](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut) — splitting mutable access
- [String::from_utf8_lossy](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy) — converting bytes to string

