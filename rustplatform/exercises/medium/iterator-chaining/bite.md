# Iterator Chaining

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, you can chain operations in a list comprehension or with `itertools`:

```python
# Filter and transform in one comprehension
result = [x * 2 for x in data if x > 0]

# Multiple steps with itertools
from itertools import chain
flat = list(chain.from_iterable(nested_lists))
longest = sorted(words, key=len)[-1]
```

Rust takes the chaining idea further — every iterator adapter returns a new iterator, so you can build multi-step pipelines that read top-to-bottom, like Unix pipes. And because each step is lazy, Rust fuses them into efficient single-pass execution.

### Building pipelines

Each method returns a new iterator that wraps the previous one. Nothing executes until a consuming adapter (`.collect()`, `.sum()`, etc.) drives the chain:

```rust
let result: Vec = data.iter()
    .filter(|&&x;| x > 0)      // keep positives
    .map(|&x;| x * 2)           // double each
    .collect();                 // materialize
```

This is equivalent to `[x * 2 for x in data if x > 0]` in Python — but each step is a separate, composable method. You can add or remove steps without restructuring the expression.

### `.take(n)` and `.skip(n)` — lazy slicing

Python uses slicing: `data[:3]`, `data[2:]`. Rust's equivalent uses lazy iterator adapters:

```rust
let first_three: Vec<_> = data.iter().take(3).collect();
let skip_two: Vec<_> = data.iter().skip(2).collect();
let middle: Vec<_> = data.iter().skip(2).take(3).collect();
```

The key advantage: `.take()` stops consuming the iterator as soon as it has enough elements. On a large or infinite iterator, this is essential — slicing requires the whole collection to exist in memory.

### `.flatten()` — unwrap nested structures

`.flatten()` takes an iterator of iterators (or iterable things) and produces a single flat iterator. It's Python's `itertools.chain.from_iterable`:

```python
# Python
from itertools import chain
flat = list(chain.from_iterable([[1, 2], [3, 4]]))  # [1, 2, 3, 4]
```


```rust
// Rust
let flat: Vec = vec![vec![1, 2], vec![3, 4]]
    .into_iter()
    .flatten()
    .collect();
// [1, 2, 3, 4]
```


### `.enumerate()` — index access

Just like Python's `enumerate()`:

```rust
for (i, value) in data.iter().enumerate() {
    println!("{i}: {value}");
}
```


### Clamping values with `.max()` and `.min()`

Rust's integer types have `.max()` and `.min()` methods for bounding values:

```rust
let floored = (score - penalty).max(0);  // never goes below 0
```

This is like Python's `max(score - penalty, 0)`, but called as a method on the value. You can chain both for a range: `value.max(0).min(100)` keeps a value between 0 and 100.

### Custom sorting

Rust's `.sort()` sorts ascending by default. For descending or custom ordering, use `.sort_by()`:

```rust
let mut words = vec!["hi", "hello", "hey"];
words.sort_by_key(|w| w.len());  // by length: ["hi", "hey", "hello"]
```

The closure returns a key to sort by. For full control, `.sort_by()` takes a closure that receives two elements and returns an `Ordering` (`Less`, `Equal`, `Greater`) — swapping the arguments reverses the order. Both sort in place and require a mutable `Vec`.

---


## Your Task


1. `process_scores(scores: &[i32])` — filter scores >= 60, add 10 bonus points, cap at 100
2. `flatten_and_sort(nested: Vec<Vec<i32>>)` — flatten nested Vecs, remove duplicates, sort ascending
3. `top_n(items: &[i32], n: usize)` — return the `n` largest items in descending order


---


## Example


```rust
assert_eq!(process_scores(&[55, 70, 95, 40]), vec![80, 100]);
assert_eq!(flatten_and_sort(vec![vec![3, 1], vec![2, 1]]), vec![1, 2, 3]);
assert_eq!(top_n(&[5, 2, 8, 1, 9], 3), vec![9, 8, 5]);
```


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — adapters and consumers
- [Iterator::flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) — unwrapping nested iterators
- [Iterator::take](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take) — lazy limiting
- [slice::sort_by](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) — custom comparison sorting

