# Variables and Mutability

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, every variable is mutable by default:

```python
total = 0
for i in range(4):
    total += i
# total is now 6
```

No special syntax needed — you just reassign. Rust takes the opposite approach: **variables are immutable by default.** If you want to change a value, you must explicitly opt in with `mut`:

```rust
let counter = 1;       // immutable — can't change this
let mut counter = 1;   // mutable — now you can
```

This isn't a restriction for the sake of it. Immutability by default means the compiler catches accidental mutations — a class of bug that Python can't detect until runtime.

### Loops in Rust

Rust's `for` loop works over ranges and iterators:

```rust
for _ in 0..5 {
    // runs 5 times (0, 1, 2, 3, 4)
}
```

`0..5` is a half-open range (excludes 5), like Python's `range(5)`. The `_` means "I don't need the loop variable."

---


## Your Task

Implement the `double_counter` function:

- Create a mutable counter variable starting at `1`
- Double it 5 times in a loop using a range
- Return the final result (should be `32`)

Don't hardcode the return value — use a mutable variable and a loop.

---


## Example


```rust
assert_eq!(double_counter(), 32);
```


---


## Further Reading


- [The Rust Book — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — let, let mut, and const
- [The Rust Book — Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops) — for, while, and loop

