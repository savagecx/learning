# Iterator Basics

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, iteration is simple — `for x in collection` works on anything iterable, and you never think about whether you're borrowing or consuming the data:

```python
nums = [1, 2, 3]
for x in nums:
    print(x)
# nums is still here — Python uses reference counting
```

You can also mutate in place:

```python
nums[0] = 99  # direct index assignment
```

Rust has the same `for x in collection` syntax, but because there's no garbage collector, you need to choose *how* you access each element — borrow it, mutate it, or take ownership of it.

### Three iterator methods — matching the ownership model

This mirrors the three capture modes you saw with closures. Collections offer three ways to iterate:
**`.iter()`** — borrows each element as `&T` (read-only):

```rust
let v = vec![1, 2, 3];
for x in v.iter() {
    println!("{x}");  // x is &i32;
}
// v is still usable — we only borrowed
```

**`.iter_mut()`** — borrows each element as `&mut T` (read-write):

```rust
let mut v = vec![10, 20, 30];
let first = v.iter_mut().next().unwrap();
*first = 99;
// v is now [99, 20, 30]
```

Note the `*first` — since `iter_mut()` yields `&mut i32` references, you dereference with `*` to modify the value behind the reference. In a `for` loop over `iter_mut()`, each `x` is a mutable reference you can write through.
**`.into_iter()`** — takes ownership of each element (`T`):

```rust
let v = vec![String::from("a"), String::from("b")];
for s in v.into_iter() {
    println!("{s}");  // s is String, not &String;
}
// v is consumed — can't use it anymore
```

The compiler picks `.into_iter()` when you write `for x in v` directly. Use `.iter()` or `.iter_mut()` when you want to keep the collection.

### Lazy evaluation

Rust iterators are **lazy** — chaining methods like `.filter()` or `.map()` doesn't do any work until you *consume* the iterator (with `for`, `.collect()`, `.sum()`, etc.). This is similar to Python generators:

```python
# Python generator — lazy, computes on demand
squares = (x**2 for x in range(1000000))
```


```rust
// Rust iterator — also lazy, no allocation until consumed
let squares = (0..1000000).map(|x| x * x);
```

The difference: Rust's standard iterator methods (`.map()`, `.filter()`, `.take()`, etc.) are all lazy by default. In Python, `map()` and `filter()` are lazy, but list comprehensions eagerly allocate. In Rust, nothing runs until you consume.

### Consuming adapters

Methods that drive the iterator to completion are called **consuming adapters**:

- `.sum()` — adds up all elements
- `.collect()` — gathers results into a collection
- `.count()` — counts elements
- `.for_each()` — runs a closure on each element

These consume the iterator — once called, it's done.

---


## Your Task


1. `sum_all(v: &[i32])` — sum all elements using `.iter()`
2. `double_all(v: &mut [i32])` — double each element in place using `.iter_mut()`
3. `into_strings(v: Vec<i32>)` — convert `Vec<i32>` to `Vec<String>` using `.into_iter()`

Notice how the function signatures match the iterator type: `&[i32]` pairs with `.iter()`, `&mut [i32]` with `.iter_mut()`, and `Vec<i32>` (owned) with `.into_iter()`.

---


## Example


```rust
assert_eq!(sum_all(&[1, 2, 3, 4]), 10);

let mut nums = vec![1, 2, 3];
double_all(&mut; nums);
assert_eq!(nums, vec![2, 4, 6]);

assert_eq!(into_strings(vec![1, 2, 3]), vec!["1", "2", "3"]);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll iterate over token streams and JSON arrays — using `.iter()`, `.enumerate()`, and consuming iterators to process parsed data.


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — the iterator trait and consuming/iterator adapters
- [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — full list of iterator methods
- [IntoIterator trait](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) — how `for` loops work under the hood

