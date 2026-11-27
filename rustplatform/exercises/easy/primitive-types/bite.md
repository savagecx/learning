# Primitive Types

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you don't declare types — the interpreter figures them out at runtime:

```python
x = 99        # int (arbitrary precision)
g = 9.81      # float (64-bit)
flag = False  # bool
ch = "A"      # str (no separate char type)
pair = (3, "Python")  # tuple
```

Rust also infers types from values, but it's **statically typed** — types are checked at compile time, not runtime. The key differences:

- **Integers have fixed sizes**: `i32` (32-bit signed), `u8` (8-bit unsigned), etc. No arbitrary precision by default.
- **Floats**: `f64` (default, same as Python's `float`) or `f32`. 
- **`char`**: a separate type for a single Unicode scalar value, written with single quotes `'A'` (not double quotes).
- **Tuples**: fixed-size, can mix types like `(i32, &str)`.


### Type inference

Rust infers types when it can:

```rust
let x = 42;      // inferred as i32 (the default integer type)
let pi = 3.14;   // inferred as f64
```

When the compiler can't infer (ambiguous context), you add a type annotation:

```rust
let x: i32 = 42;
```


### The format! macro

Rust's `format!` is like Python's `f"..."` — it builds a string from values:

```rust
format!("x is {}, pi is {}", x, pi)
// equivalent to Python's f"x is {x}, pi is {pi}"
```

Use `{:?}` for debug formatting (useful for tuples and complex types).

---


## Your Task

Implement `describe_types` — create variables for each primitive type and return a formatted string:

- An integer (`i32`): `42`
- A float (`f64`): `3.14`
- A boolean: `true`
- A character: `'Z'`
- A tuple: `(7, "Rust")`

Return the string: `int: 42, float: 3.14, bool: true, char: Z, tuple: (7, "Rust")`
Use `format!` with `{:?}` for the tuple.

---


## Example


```rust
let output = describe_types();
assert!(output.contains("int: 42"));
assert!(output.contains("float: 3.14"));
assert!(output.contains("tuple: (7, \"Rust\")"));
```


---


## Further Reading


- [The Rust Book — Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) — scalars, compounds, and type annotations
- [std::fmt](https://doc.rust-lang.org/std/fmt/index.html) — the format! macro and formatting syntax

