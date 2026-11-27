# Implementing Custom Iterators

- Level: hard
- Author: Pybites

## Instructions
🎯 In Python, you make any object iterable by implementing `__iter__` and `__next__`:

```python
class Countdown:
    def __init__(self, start):
        self.current = start

    def __iter__(self):
        return self

    def __next__(self):
        if self.current <= 0:
            raise StopIteration
        val = self.current
        self.current -= 1
        return val
```

Or more commonly, you use a generator:

```python
def countdown(start):
    while start > 0:
        yield start
        start -= 1
```

Rust uses the same idea — a trait with a `next()` method — but the payoff is bigger. Implement one method, and you get 70+ iterator methods (`.map()`, `.filter()`, `.take()`, `.sum()`, `.collect()`, etc.) for free.

### The `Iterator` trait

To make your type iterable, implement the `Iterator` trait:

```rust
struct Countdown {
    current: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut; self) -> Option {
        if self.current > 0 {
            let val = self.current;
            self.current -= 1;
            Some(val)
        } else {
            None
        }
    }
}
```

Two things to note:
**`type Item`** is an *associated type* — it declares what this iterator yields. Python doesn't have an equivalent; `__next__` can return anything. In Rust, the type is fixed at compile time.
**`fn next(&mut self)`** takes a mutable reference to `self` because iterators are stateful — each call advances internal state. This is just like Python's `__next__` modifying `self.current`.
**`Some(value)` vs `None`** replaces Python's `StopIteration`. Return `Some(value)` for the next item, `None` to signal "I'm done." No exceptions, just data.

### One method, 70+ free

Once you implement `next()`, your type automatically gets every method from the `Iterator` trait:

```rust
let sum: u32 = Countdown { current: 5 }.sum();  // 15
let v: Vec = Countdown { current: 3 }
    .map(|x| x * 10)
    .collect();  // [30, 20, 10]
```

In Python, you'd need to wrap your iterator with `map()`, `filter()`, or `itertools` functions. In Rust, these are methods directly on your type — because it implements `Iterator`.

### Infinite iterators

Some iterators never end. A cycle iterator, for example, loops forever. This is safe because Rust iterators are lazy — an infinite iterator only produces values when asked:

```rust
let first_five: Vec<_> = my_cycle.take(5).collect();
```

`.take(5)` limits the iterator to 5 elements. Without a consuming limit, you'd loop forever — the compiler won't stop you, so it's your responsibility to use `.take()`, `.take_while()`, or break out of a `for` loop.

### Lifetimes on iterators that borrow

When your iterator borrows data (like a slice), it needs a **lifetime parameter** to tell the compiler how long the borrowed data lives:

```rust
struct Windows<'a, T> {
    data: &'a [T],
    pos: usize,
    size: usize,
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut; self) -> Option {
        // ...
    }
}
```

The `'a` says: "this iterator can't outlive the slice it borrows." The `type Item = &'a [T]` says: "each yielded value is a reference that lives as long as the original data." Python doesn't need this because the garbage collector tracks references automatically. In Rust, lifetimes make the same guarantee at compile time.
Don't worry if lifetimes feel unfamiliar — they're covered in depth in the lifetimes track.

---


## Your Task


1. `Range` — iterate from `start` (inclusive) to `end` (exclusive), like Python's `range()`
2. `Fibonacci` — generate `n` Fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, ...
3. `Cycle` — endlessly cycle through a borrowed slice, returning references

The struct definitions and signatures are provided in the template. You need to implement `new()` and `next()` for each.

---


## Example


```rust
let r: Vec = Range::new(1, 4).collect();
assert_eq!(r, vec![1, 2, 3]);

let fib: Vec = Fibonacci::new(7).collect();
assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8]);

let mut cycle = Cycle::new(&[1, 2, 3]);
assert_eq!(cycle.next(), Some(&1));
assert_eq!(cycle.next(), Some(&2));
assert_eq!(cycle.next(), Some(&3));
assert_eq!(cycle.next(), Some(&1));  // cycles back

// All iterator methods work:
let sum: i32 = Range::new(1, 11).sum();  // 55
let fib_sum: u64 = Fibonacci::new(10).sum();  // 88
let five: Vec<&i32;> = Cycle::new(&[1, 2]).take(5).collect();
```


---


## Further Reading


- [The Rust Book — Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — implementing the Iterator trait
- [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — all 70+ provided methods
- [The Rust Book — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) — understanding `'a`

