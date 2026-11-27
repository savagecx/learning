# Implementing Debug

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, every object has `__repr__` for developer-friendly output:

```python
class Color:
    def __init__(self, r, g, b):
        self.r, self.g, self.b = r, g, b

    def __repr__(self):
        return f"Color(r={self.r}, g={self.g}, b={self.b})"

print(repr(Color(255, 0, 0)))  # Color(r=255, g=0, b=0)
```

If you don't implement `__repr__`, Python gives you something unhelpful like `<Color object at 0x...>`. So most Python developers learn early to define `__repr__` on every class.
Rust's equivalent is the `Debug` trait. It's what powers `{:?}` formatting — the format specifier you'll use constantly during development:

```rust
println!("{:?}", my_value);   // Debug output
println!("{:#?}", my_value);  // Pretty-printed Debug output (indented)
```


### Deriving Debug

Here's where Rust differs from Python: you don't have to write any formatting code. The `derive` macro generates `Debug` automatically:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

let origin = Point { x: 0.0, y: 0.0 };
println!("{:?}", origin);  // Point { x: 0.0, y: 0.0 }
```

One line — `#[derive(Debug)]` — and the compiler writes the `__repr__` equivalent for you. The derived output includes the struct name and all fields. Python's `dataclasses.dataclass` does something similar with auto-generated `__repr__`, but Rust's derive works on any struct.
The requirement: all fields must themselves implement `Debug`. Standard types (`u8`, `i32`, `String`, `char`, `Vec<T>` where T: Debug) all do.

### Manual Debug implementation

Sometimes the derived format isn't what you want. A `Score` struct might derive as `Score { player: "Alice", points: 42 }` but you'd prefer `Alice: 42 pts`.
For custom formatting, you implement `fmt::Debug` manually:

```rust
use std::fmt;

impl fmt::Debug for Score {
    fn fmt(&self;, f: &mut; fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} pts", self.player, self.points)
    }
}
```

The signature: `fmt` takes `&self` and a `Formatter` (the output buffer), returns `fmt::Result`. The `write!` macro writes formatted text into the formatter — it works like `format!` but writes to a destination instead of creating a new `String`.
This is analogous to Python's:

```python
def __repr__(self):
    return f"{self.player}: {self.points} pts"
```

The difference: Rust's `write!` writes directly to the formatter buffer instead of allocating a new string. This matters when you're formatting nested structures — each level writes into the same buffer instead of creating intermediate strings.

### When to derive vs. implement manually


- **Derive** when the default `StructName { field: value, ... }` format is fine (most of the time)
- **Implement manually** when you want a specific format, want to hide fields, or need to transform values in the output


---


## Your Task


1. Add `#[derive(Debug)]` to the `Color` struct so it can be formatted with `{:?}`
2. Implement `Debug` for `Temperature` manually — format it as `"Temperature(VALUE UNIT)"` (e.g., `"Temperature(25 C)"` or `"Temperature(77 F)"`)


---


## Example


```rust
let red = Color { r: 255, g: 0, b: 0 };
println!("{:?}", red);  // Color { r: 255, g: 0, b: 0 }

let temp = Temperature { value: 25, unit: 'C' };
println!("{:?}", temp); // Temperature(25 C)

let cold = Temperature { value: -10, unit: 'C' };
println!("{:?}", cold); // Temperature(-10 C)
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), `#[derive(Debug)]` on your `Token` and `JsonValue` types lets you inspect the parser's output with `{:?}` — essential for development and testing.


---


## Further Reading


- [The Rust Book — Printing with Debug](https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits) — deriving Debug on structs
- [std::fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) — the Debug trait documentation
- [std::fmt module](https://doc.rust-lang.org/std/fmt/index.html) — formatting traits overview (Debug, Display, and more)

