# Fold and Reduce

- Level: medium
- Author: Pybites

## Instructions
🎯 Python has several ways to reduce a collection to a single value:

```python
from functools import reduce

sum([1, 2, 3, 4])                              # 10
reduce(lambda acc, s: acc + s, ["a", "b"], "")  # "ab"
max([3, 1, 4, 1, 5])                           # 5
```

Rust has the same concepts, but with an important distinction between `fold` (always has an initial value) and `reduce` (uses the first element). That distinction matters because Rust handles empty collections differently.

### `.fold(init, f)` — accumulate with an initial value

`.fold()` is Python's `reduce` with an explicit starting value. It takes an initial accumulator and a closure that combines the accumulator with each element:

```rust
let sum = vec![1, 2, 3, 4].iter().fold(0, |acc, x| acc + x);
// 0 + 1 + 2 + 3 + 4 = 10

let joined = vec!["a", "b", "c"].iter()
    .fold(String::new(), |acc, s| acc + s);
// "abc"
```

The initial value is the **identity element** for your operation: `0` for addition, `1` for multiplication, `""` for string concatenation. On an empty collection, `.fold()` returns the initial value — no special handling needed.
This is why `.fold()` returns `T` directly, not `Option<T>`. It always has a result.

### `.reduce(f)` — use the first element as the initial value

`.reduce()` is like `.fold()` but takes the first element as the starting accumulator. Since the collection might be empty — no first element — it returns `Option<T>`:

```rust
let max = vec![1, 5, 3].into_iter()
    .reduce(|a, b| if a > b { a } else { b });
// Some(5)

let empty: Vec = vec![];
let max = empty.into_iter()
    .reduce(|a, b| if a > b { a } else { b });
// None — no elements to reduce
```

In Python, `reduce` on an empty iterable without an initial value raises `TypeError`. Rust returns `None` instead — same problem, different error-handling philosophy.

### `.scan()` — fold that yields intermediate results

`.scan()` is like `.fold()` but produces a value at each step — a running accumulation. It's the equivalent of Python's `itertools.accumulate`:

```python
# Python
from itertools import accumulate
import operator
list(accumulate([2, 3, 4], operator.mul))  # [2, 6, 24]
```


```rust
// Rust
let running_product: Vec = vec![2, 3, 4].iter()
    .scan(1, |acc, &x;| {
        *acc *= x;
        Some(*acc)
    })
    .collect();
// [2, 6, 24]
```

The closure receives a mutable reference to the accumulator (`&mut acc`) and returns `Option<T>`. Returning `Some(value)` yields that value; returning `None` stops the iteration. The `*acc` dereference is needed because `acc` is `&mut i32` — you're modifying state through a reference, just like with `.iter_mut()`.

### Common shortcuts

For the most common aggregations, Rust provides dedicated methods so you don't need `.fold()`:

- `.sum()` — add all elements (requires `Sum` trait)
- `.product()` — multiply all elements (requires `Product` trait)
- `.max()` / `.min()` — find the largest or smallest (returns `Option<T>`)
- `.count()` — count the number of elements

These are consuming adapters — they drive the iterator to completion.

---


## Your Task


1. `product(v: &[i32])` — multiply all elements together using `.fold()` (return `1` for empty slices)
2. `longest_string(v: &[&str])` — find the longest string using `.reduce()` (prefer the first if tied)
3. `running_total(v: &[i32])` — return a `Vec<i32>` of running totals: `[1, 2, 3]` → `[1, 3, 6]`


---


## Example


```rust
assert_eq!(product(&[2, 3, 4]), 24);
assert_eq!(product(&[]), 1);

assert_eq!(longest_string(&["hi", "hello", "hey"]), Some("hello"));
assert_eq!(longest_string(&[]), None);

assert_eq!(running_total(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
```


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — consuming adapters including fold
- [Iterator::fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) — accumulate with an initial value
- [Iterator::reduce](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce) — accumulate using the first element
- [Iterator::scan](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) — stateful map yielding intermediate results

