# Closure Captures

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, closures can read variables from their enclosing scope freely, but *modifying* them requires the `nonlocal` keyword:

```python
def toggler():
    state = False
    def flip():
        nonlocal state
        state = not state
        return state
    return flip

t = toggler()
t()  # True
t()  # False
```

Without `nonlocal`, Python treats `state` as a new local variable and you get an `UnboundLocalError`. Rust handles this differently — through three capture modes and three closure traits.

### Three ways to capture

Closures can capture variables from their environment in three ways:
**By reference (`&T`)** — default when the closure only reads:

```rust
let x = 5;
let read_x = || println!("{x}");  // borrows x
read_x();
println!("{x}");  // x still usable
```

**By mutable reference (`&mut T`)** — when the closure modifies:

```rust
let mut flag = false;
let mut toggle = || flag = !flag;  // mutably borrows flag
toggle();
```

**By value (`move`)** — takes ownership:

```rust
let s = String::from("hello");
let owns_s = move || println!("{s}");  // takes ownership of s
owns_s();
// s is no longer valid here
```

The compiler picks the *least restrictive* mode automatically. It only borrows mutably if the closure modifies the value, and only moves if you add the `move` keyword (or the closure must outlive the captured data).

### Three closure traits: `Fn`, `FnMut`, `FnOnce`

The capture mode determines which trait the closure implements:
| Trait | What it means | Can call multiple times? |
|-------|--------------|------------------------|
| `Fn` | Only reads captured values | Yes |
| `FnMut` | May modify captured values | Yes (but needs `mut`) |
| `FnOnce` | May consume captured values | Once only |
Every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce` — they form a hierarchy. The template signatures tell you which trait each function needs to return.

### The caller needs `mut` too

When a closure implements `FnMut`, the *caller* must also declare the binding as mutable:

```rust
let mut count = counter();  // mut because count() modifies internal state
count();
count();
```

This is Rust being explicit — the `mut` signals that calling this closure has side effects.

---


## Your Task


1. `counter()` — return a closure that increments and returns a count each time it's called (1, 2, 3, ...)
2. `create_greeting(name)` — return a closure that returns `"Hello, {name}!"`
3. `accumulator(initial)` — return a closure that adds to a running total and returns it


---


## Example


```rust
let mut count = counter();
assert_eq!(count(), 1);
assert_eq!(count(), 2);

let greet = create_greeting(String::from("Alice"));
assert_eq!(greet(), "Hello, Alice!");

let mut acc = accumulator(10);
assert_eq!(acc(5), 15);   // 10 + 5
assert_eq!(acc(3), 18);   // 15 + 3
```


---


## Further Reading


- [The Rust Book — Closures: Anonymous Functions that Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html) — capture modes and the `Fn` traits
- [Fn vs FnMut vs FnOnce](https://doc.rust-lang.org/std/ops/trait.Fn.html) — the trait hierarchy explained

