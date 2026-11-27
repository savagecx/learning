# Map and Filter

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, list comprehensions handle both transforming and filtering in one readable expression:

```python
squares = [x**2 for x in numbers]
evens = [x for x in numbers if x % 2 == 0]
lengths = [len(s) for s in words if s]
```

Python also has `map()` and `filter()` builtins, but list comprehensions are more idiomatic. In Rust, iterator methods like `.map()` and `.filter()` *are* the idiomatic approach — they replace both loops and comprehensions.

### `.map()` — transform each element

`.map()` takes a closure and applies it to every element, producing a new iterator:

```rust
let doubled: Vec = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6]
```

This is like `[x * 2 for x in vec]` in Python. The closure receives each element (here `&i32` since we used `.iter()`), and the result becomes the new element.

### `.filter()` — keep elements matching a condition

`.filter()` takes a predicate (a closure returning `bool`) and keeps only elements where it returns `true`:

```rust
let evens: Vec<&i32;> = vec![1, 2, 3, 4]
    .iter()
    .filter(|x| *x % 2 == 0)
    .collect();
// [&2, &4]
```

**Watch the references.** When you call `.filter()` on an `.iter()`, there's a double reference: `.iter()` yields `&i32`, and `.filter()` passes each element *by reference*, so the closure gets `&&i32`. You'll need to dereference with `*` or use pattern matching (`|&&x|`) to access the value. This is a common stumbling block — if the compiler complains about applying `%` to `&&i32`, this is why.
To get owned values out, you can use `.copied()` after filtering (for types that implement `Copy`):

```rust
let evens: Vec = vec![1, 2, 3, 4]
    .iter()
    .filter(|x| **x % 2 == 0)
    .copied()
    .collect();
// [2, 4] — now Vec, not Vec<&i32;>
```


### `.filter_map()` — filter and transform in one step

Sometimes you want to try a transformation that might fail, keeping only the successes. In Python:

```python
# keep only non-empty first characters
firsts = [s[0] for s in ["hello", "", "world"] if s]
# ['h', 'w']
```

Rust's `.filter_map()` combines `.filter()` and `.map()` — the closure returns `Option<T>`, and only `Some` values are kept:

```rust
let first_chars: Vec = vec!["hello", "", "world"]
    .iter()
    .filter_map(|s| s.chars().next())
    .collect();
// ['h', 'w'] — empty string yields None, so it's skipped
```

The closure returns `Option<T>` — `Some(value)` is kept, `None` is dropped. This is cleaner than a separate `.filter()` + `.map()` chain when the transform itself tells you whether to keep the element.

### Chaining

All these methods are lazy iterators, so you can chain them freely. Nothing runs until a consuming adapter (`.collect()`, `.sum()`, etc.) drives the chain:

```rust
let result: Vec = numbers.iter()
    .filter(|&&x;| x > 0)
    .map(|&x;| x * x)
    .collect();
```


---


## Your Task


1. `squares(v: &[i32])` — return a `Vec<i32>` of each number squared
2. `only_positive(v: &[i32])` — return only the numbers greater than zero
3. `parse_numbers(v: &[&str])` — parse each string to `i32`, keeping only the ones that parse successfully


---


## Example


```rust
assert_eq!(squares(&[1, 2, 3]), vec![1, 4, 9]);
assert_eq!(only_positive(&[-1, 0, 1, 2]), vec![1, 2]);
assert_eq!(parse_numbers(&["1", "two", "3"]), vec![1, 3]);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll use `.map()` and `.filter()` to transform and validate tokens, and `.filter_map()` to skip invalid entries while parsing JSON arrays.


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — map, filter, and the iterator pipeline
- [Iterator::filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) — combining filter and map
- [Result::ok](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) — converting Result to Option

