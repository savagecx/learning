# Vec Operations

- Level: medium
- Author: Pybites

## Instructions
In Python, lists are your go-to collection:

```python
items = [1, 2, 3]
items.append(4)
items.extend([5, 6])
first = items.pop(0)
items.insert(0, first)
```

Rust's `Vec<T>` is similar but with explicit types and ownership semantics.

### Creating vectors


```rust
let mut v: Vec = Vec::new();
let v = vec![1, 2, 3];  // macro for convenience
```


### Adding elements


```rust
let mut v = vec![1, 2, 3];
v.push(4);              // add to end
v.extend([5, 6]);       // add multiple
v.insert(0, 0);         // insert at index
```


### Removing elements


```rust
let last = v.pop();           // Option - might be empty
let removed = v.remove(0);    // panics if out of bounds
v.retain(|&x;| x > 2);         // keep only matching elements
```


### Accessing elements


```rust
let first = v[0];             // panics if empty
let first = v.get(0);         // Option<&T;> - safe
let first = v.first();        // Option<&T;>
let last = v.last();          // Option<&T;>
```


### Slices: borrowed views


```rust
let slice: &[i32] = &v;[1..3];  // borrow a portion
let all: &[i32] = &v;          // borrow entire vec
```

Functions that don't need ownership should take `&[T]` (slice) instead of `&Vec<T>` - more flexible.

---


## Your Task


1. `merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32>` — merge two already-sorted slices into one sorted `Vec`
2. `chunk_vec(items: Vec<i32>, size: usize) -> Vec<Vec<i32>>` — split into chunks of the given size (last chunk may be smaller)
3. `partition_by_predicate(items: Vec<i32>, threshold: i32) -> (Vec<i32>, Vec<i32>)` — split into (below threshold, at or above threshold)


---


## Example


```rust
assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);

assert_eq!(chunk_vec(vec![1, 2, 3, 4, 5], 2), vec![vec![1, 2], vec![3, 4], vec![5]]);

let (below, above) = partition_by_predicate(vec![1, 5, 2, 8, 3], 4);
assert_eq!(below, vec![1, 2, 3]);
assert_eq!(above, vec![5, 8]);
```


---


## Further Reading


- [The Rust Book — Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Vec in std](https://doc.rust-lang.org/std/vec/struct.Vec.html)

