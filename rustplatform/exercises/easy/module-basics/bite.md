# Module Basics

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you organize code into modules (files) and packages (directories with `__init__.py`):

```python
# converter/units.py
def miles_to_km(miles):
    return miles * 1.60934

def kg_to_lb(kg):
    return kg * 2.20462
```


```python
# main.py
from converter.units import miles_to_km
```

Everything in a Python module is public by default. You use `_prefixes` as a convention to signal "private," but nothing enforces it.
Rust uses a `mod` system that's similar in structure but different in philosophy: **everything is private by default**, and you opt in to making things public with `pub`.

### Defining modules with `mod`

In Rust, modules are declared with the `mod` keyword. They can be inline (in the same file) or in separate files:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn internal_helper() {
        // private — only visible inside this module
    }
}

let sum = math::add(1, 2);  // OK
// math::internal_helper();  // ERROR: private function
```

The `pub` keyword is required for anything you want to expose. Without it, functions, structs, and constants are only visible within the module they're defined in.
In Python, `from module import *` grabs everything. In Rust, only `pub` items are visible outside the module. Privacy is enforced by the compiler, not by convention.

### Nested modules

Modules can contain other modules, like Python's subpackages:

```rust
mod outer {
    pub mod inner {
        pub fn greet() -> &'static str {
            "Hello from inner!"
        }
    }
}

let msg = outer::inner::greet();
```

The path syntax uses `::` (like Python's `.` but for modules and types). Note that the nested module itself must be `pub` for outer code to access it.

### The `use` statement

Rust's `use` is like Python's `from ... import`:

```rust
use std::f64::consts::PI;

let circumference = 2.0 * PI * 5.0;
```

Without `use`, you'd write the full path every time: `std::f64::consts::PI`. The `use` statement brings items into scope — just like Python's imports.

### File-based modules

In real projects, modules map to files. `mod math;` (without a body) tells Rust to look for `math.rs` or `math/mod.rs`. This is covered in later exercises — for now, we'll use inline modules.

---


## Your Task


1. Create a `geometry` module with:
2. `pub fn area_rectangle(w: f64, h: f64) -> f64`
3. `pub fn area_circle(r: f64) -> f64` (use `std::f64::consts::PI`)
4. Inside `geometry`, create a nested `helpers` module with:
5. `pub fn square(x: f64) -> f64` (returns x * x)


---


## Example


```rust
assert_eq!(geometry::area_rectangle(3.0, 4.0), 12.0);
assert!((geometry::area_circle(1.0) - 3.14159).abs() < 0.001);
assert_eq!(geometry::helpers::square(5.0), 25.0);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), your JSON parser will be organized into modules — `tokenizer`, `parser`, `error`, `value` — each with clear responsibilities and public APIs.


---


## Further Reading


- [The Rust Book — Defining Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) — the module system and privacy
- [The Rust Book — Paths for Referring to an Item](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) — `::` path syntax
- [The Rust Book — Bringing Paths into Scope with use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html) — `use` statements

