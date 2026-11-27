# Variable Assignment and Mutability

- Level: easy
- Author: dpdresser

## Instructions
🎯 In Python, all variables are mutable by default:

```python
x = 5
x += 1  # no problem
```

Rust flips this: variables are **immutable by default**. This is one of the biggest mindset shifts for Python developers.

```rust
let x = 5;
x += 1;  // ERROR: cannot assign twice to immutable variable
```

The compiler even tells you how to fix it — add `mut`:

```rust
let mut x = 5;
x += 1;  // now it works
```

Why immutable by default? It prevents accidental mutations. When you see `let x = ...` without `mut`, you *know* that value never changes — no need to trace through the code to check. It's like a built-in guarantee that Python can't offer.

### Pass by value vs pass by reference

In Python, function arguments are passed by "assignment" — the function gets a reference to the same object. Mutating a list inside a function affects the original.
In Rust, simple types like `i32` implement the **`Copy` trait**, which means they're *copied* when passed to a function:

```rust
fn double(x: i32) -> i32 {
    x * 2  // works on a copy; the original is untouched
}

let n = 5;
let result = double(n);
// n is still 5, result is 10
```

To modify a value *in place*, you pass a **mutable reference** with `&mut`:

```rust
fn negate_in_place(x: &mut; i32) {
    *x = -*x;
}

let mut n = 5;
negate_in_place(&mut; n);
// n is now -5
```

Notice the `*x` — that's the **dereference operator**. `x` is a reference (a pointer to the value), so `*x` accesses the actual value behind it. Python handles this indirection automatically; Rust makes you spell it out.

### The unit type `()`

When a function returns nothing (like a Python function with no `return`), Rust uses the **unit type** `()`:

```rust
fn do_something(x: &mut; i32) -> () {
    *x *= 2;
}
```

You can also omit `-> ()` entirely — it's the default when no return type is specified.

---


## Your Task

Implement two functions:

1. `increment_by_one(x: i32) -> i32` — takes a value, returns it incremented by 1 (the original is untouched)
2. `increment_mutable(x: &mut i32) -> ()` — takes a mutable reference and increments the value in place


---


## Example


```rust
assert_eq!(increment_by_one(5), 6);
assert_eq!(increment_by_one(-20), -19);

let mut value = 5;
increment_mutable(&mut; value);
assert_eq!(value, 6);
```


---


## Further Reading


- [The Rust Book — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — `let`, `mut`, and why immutability is the default
- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — `&` vs `&mut` and the dereference operator

