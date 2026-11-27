# From and Into Traits

- Level: easy
- Author: Pybites

## Instructions
🎯 Python handles type conversions with constructors and dunder methods:

```python
n = int("42")              # str → int via constructor
f = float(42)              # int → float via constructor

class Celsius:
    def __init__(self, value):
        self.value = value

    def __float__(self):   # supports float(celsius_obj)
        return self.value
```

Each conversion is ad-hoc. `int()` works differently from `float()`, and custom classes define whatever dunder methods they want. There's no single "conversion protocol" that the whole ecosystem agrees on.
Rust has one: the `From` trait.

### The From trait

`From<T>` defines how to create your type from some other type `T`:

```rust
impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color { r: rgb.0, g: rgb.1, b: rgb.2 }
    }
}

let red = Color::from((255, 0, 0));
```

The signature is always the same: `fn from(value: T) -> Self`. One input, one output, no ambiguity. Every conversion in the Rust ecosystem follows this pattern — standard library types, third-party crates, your own code.

### Into: the reverse direction

Implementing `From` automatically gives you `Into` for free (via a blanket implementation in the standard library). So if `Point` implements `From<(i32, i32)>`, then `(i32, i32)` automatically implements `Into<Point>`:

```rust
// Both work after implementing From:
let c1 = Color::from((255, 0, 0));     // explicit From
let c2: Color = (0, 255, 0).into();    // implicit Into (needs type annotation)
```

`.into()` is useful when the target type is clear from context. The type annotation (`: Color`) tells Rust which `Into` implementation to use — since a tuple could `.into()` many different types.
**Rule of thumb:** implement `From`, use `.into()`. Never implement `Into` directly.

### Tuple structs and .0 access

The exercise uses tuple structs — structs with unnamed fields accessed by position:

```rust
struct Celsius(f64);     // one-field tuple struct
struct Fahrenheit(f64);

let c = Celsius(100.0);
let value = c.0;         // access by index: 100.0
```

This is Rust's version of a newtype pattern — a thin wrapper that gives a raw value a distinct type. Python's `typing.NewType` is similar but only for type checkers. Rust's newtypes are real types that the compiler enforces.

### From in the ecosystem

`From` is everywhere in Rust because it integrates with other patterns:

- **The `?` operator** uses `From` to convert error types automatically
- **Generic functions** accepting `impl Into<String>` let callers pass `&str` or `String`
- **Standard library** implements `From` between all compatible types (`String::from("hello")`, `Vec::from([1,2,3])`)

When you implement `From` for your types, they immediately work with all these patterns.

---


## Your Task


1. Implement `From<(i32, i32)>` for `Point` — create a Point from a tuple
2. Implement `From<[i32; 2]>` for `Point` — create a Point from a fixed-size array
3. Implement `From<Celsius>` for `Fahrenheit` — temperature conversion using formula: `F = C * 9/5 + 32`

All three should also work with `.into()`.

---


## Example


```rust
// From tuple
let p1 = Point::from((3, 4));
assert_eq!(p1.x, 3);

// Into from tuple
let p2: Point = (5, 6).into();
assert_eq!(p2.y, 6);

// From array
let p3 = Point::from([7, 8]);

// Temperature conversion
let c = Celsius(100.0);
let f = Fahrenheit::from(c);
assert!((f.0 - 212.0).abs() < 0.001);  // boiling point

// Into also works for temperature
let c2 = Celsius(0.0);
let f2: Fahrenheit = c2.into();
assert!((f2.0 - 32.0).abs() < 0.001);  // freezing point
```


---


## Further Reading


- [The Rust Book — Using the From Trait](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) — From with error handling
- [std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html) — the From trait documentation
- [std::convert::Into](https://doc.rust-lang.org/std/convert/trait.Into.html) — the Into trait and blanket implementation

