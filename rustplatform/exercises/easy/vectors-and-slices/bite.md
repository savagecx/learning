# Vectors and Slices

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, lists are the go-to dynamic collection:

```python
nums = [1, 2, 3]
nums.append(4)
total = sum(nums)

# In-place mutation
nums = [x ** 2 for x in nums]
```

Rust's equivalent is `Vec<T>` — a growable, heap-allocated array. But idiomatic Rust functions often accept **slices** (`&[T]`) instead of `Vec<T>` directly. This lets them work with both vectors and arrays:

```rust
fn max_slice(v: &[i32]) -> Option {
    v.iter().copied().max()
}

let vec = vec![1, 2, 3];
let arr = [1, 2, 3];
max_slice(&vec;);   // works — Vec coerces to &[T]
max_slice(&arr;);   // works — array coerces to &[T]
```


### Mutable slices

To modify elements in place, take `&mut [T]`:

```rust
fn increment(x: &mut; i32) {
    *x += 1;    // dereference with * to modify the value
}
```

The `*x` dereference is new for Python developers — `x` is a `&mut i32` (a mutable reference to an integer), and `*x` accesses the value behind the reference. Python doesn't have this because everything is a reference behind the scenes.

### &[T] vs Vec

Think of it like Python's typing:
- `Vec<T>` ≈ `list[T]` — owning the data
- `&[T]` ≈ `Sequence[T]` — a read-only view, works with any contiguous collection
- `&mut [T]` ≈ `MutableSequence[T]` — a mutable view
Functions that only read data should take `&[T]`. Functions that modify in place should take `&mut [T]`. Functions that need to own the data take `Vec<T>`.

---


## Your Task

Implement two functions:

1. `sum_slice(v: &[i32]) -> i32` — return the sum of all elements
2. `square_in_place(v: &mut [i32])` — square each element in place


---


## Example


```rust
let mut nums = vec![1, 2, 3];
square_in_place(&mut; nums);
assert_eq!(nums, vec![1, 4, 9]);
assert_eq!(sum_slice(&nums;), 14);
assert_eq!(sum_slice(&[]), 0);
```


---


## Further Reading


- [The Rust Book — Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) — creating and using Vec
- [The Rust Book — The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html) — slices as references to sequences

