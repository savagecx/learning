# Closure Basics

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, `lambda` gives you anonymous functions, but they're limited to a single expression. For anything more, you define a regular function or a closure using nested `def`:

```python
def make_greeter(prefix):
    return lambda name: f"{prefix}, {name}!"

greet = make_greeter("Hey")
greet("Alice")  # "Hey, Alice!"
```

Rust closures are more like Python's nested `def` — they can be multi-line, capture variables from their environment, and be stored, passed, and returned.

### Closure syntax

The basic syntax uses `|params|` instead of `def` or `lambda`:

```rust
let add = |a, b| a + b;
let result = add(2, 3);  // 5

let process = |x: i32| {
    let doubled = x * 2;
    doubled + 1
};
let output = process(5);  // 11
```

Types are usually inferred, but you can annotate them: `|a: i32, b: i32| -> i32 { a + b }`.

### `move` — owning captured values

When a closure captures a variable from its environment, Rust borrows it by default. But if the closure needs to *outlive* the scope where it was created (like when returned from a function), it must **own** the captured values. That's what `move` does:

```rust
fn make_logger(tag: String) -> impl Fn() -> String {
    move || format!("[{tag}]")
    // move takes ownership of `tag` — it lives inside the closure now
}
```

Without `move`, the closure would try to borrow `tag`, but `tag` is dropped when the function returns. `move` transfers ownership into the closure so it's self-contained.

### Returning closures: `impl Fn`

In Python, returning a function is straightforward — functions are objects. In Rust, every closure has a unique, anonymous type that the compiler generates. You can't write that type out, so you use `impl Fn`:

```rust
fn make_greeter(prefix: String) -> impl Fn(&str;) -> String {
    move |name| format!("{prefix}, {name}!")
}
```

`impl Fn(&str) -> String` means "returns something that can be called with a `&str` and returns a `String`." The `Fn` trait is what makes closures callable — the compiler implements it automatically.

### Accepting closures: generics with `Fn`

To accept a closure as a parameter, use a generic with a `Fn` trait bound:

```rust
fn apply(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}
```

The `where` clause says: "F can be anything that implements `Fn(i32) -> i32`" — a closure, a function, or anything callable with that signature.

---


## Your Task


1. `make_adder(n)` — return a closure that adds `n` to its argument
2. `make_multiplier(n)` — return a closure that multiplies its argument by `n`
3. `apply_twice(f, x)` — apply closure `f` to `x` twice: `f(f(x))`


---


## Example


```rust
let add_5 = make_adder(5);
assert_eq!(add_5(10), 15);

let times_3 = make_multiplier(3);
assert_eq!(times_3(4), 12);

assert_eq!(apply_twice(|x| x + 1, 5), 7);  // 5 -> 6 -> 7
```


---


## Further Reading


- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) — syntax, capturing, and the `Fn` traits
- [The Rust Book — Returning Closures](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#returning-closures) — `impl Fn` and `move`

