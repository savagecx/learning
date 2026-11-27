# Implementing Display

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, `__str__` controls what users see when they print an object:

```python
class Fraction:
    def __init__(self, num, den):
        self.num, self.den = num, den

    def __str__(self):
        return f"{self.num}/{self.den}"

print(Fraction(3, 4))  # 3/4
```

And `__repr__` is for developers — what you see in the REPL or in debug output. Many Python developers conflate the two or only implement one.
Rust makes this separation explicit with two traits:
| Trait | Format specifier | Purpose | Python equivalent |
|-------|-----------------|---------|-------------------|
| `Debug` | `{:?}` | Developer output | `__repr__` |
| `Display` | `{}` | User-facing output | `__str__` |
You met `Debug` in the previous exercise — it can be derived with `#[derive(Debug)]`. `Display` is different: **it cannot be derived.** The compiler won't guess what user-facing output should look like, so you always implement it manually.

### Implementing Display

The signature is identical to `Debug` — you implement `fmt` on the `fmt::Display` trait:

```rust
use std::fmt;

struct Fraction {
    num: i32,
    den: i32,
}

impl fmt::Display for Fraction {
    fn fmt(&self;, f: &mut; fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

println!("{}", Fraction { num: 3, den: 4 });  // 3/4
```

The `write!` macro writes into the formatter buffer, just like with `Debug`. The only difference is which format specifier triggers it: `{}` calls `Display`, `{:?}` calls `Debug`.

### The free ToString

Here's a bonus Python developers appreciate: implementing `Display` automatically gives you `.to_string()`. Rust has a blanket implementation:

```rust
// This exists in the standard library:
impl ToString for T {
    fn to_string(&self;) -> String {
        // uses Display::fmt internally
    }
}
```

So once you implement `Display`, this works:

```rust
let f = Fraction { num: 3, den: 4 };
let s: String = f.to_string();  // "3/4"
```

In Python, `str(obj)` calls `__str__`. In Rust, `.to_string()` calls `Display::fmt`. Same concept, different syntax.

### Debug + Display together

Most types should have both. A common pattern:

```rust
#[derive(Debug)]  // derive Debug for {:?}
struct Fraction { num: i32, den: i32 }

impl fmt::Display for Fraction {  // manually implement Display for {}
    fn fmt(&self;, f: &mut; fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
```

`Debug` output: `Fraction { num: 3, den: 4 }` (verbose, shows field names — good for debugging).
`Display` output: `3/4` (clean, user-facing).
Python's `@dataclass` generates `__repr__` but not `__str__`. Rust's `#[derive(Debug)]` generates `Debug` but not `Display`. In both languages, user-facing representation requires intentional design.

---


## Your Task

Implement `Display` for two structs:

1. `Point` — format as `"(x, y)"` (e.g., `"(3, 4)"`)
2. `Person` — format as `"Name (age years old)"` (e.g., `"Alice (30 years old)"`)

Both should also work with `.to_string()` (this comes automatically from implementing `Display`).

---


## Example


```rust
let p = Point { x: 3, y: 4 };
println!("{}", p);          // (3, 4)
println!("{}", p.to_string()); // (3, 4)

let person = Person { name: String::from("Alice"), age: 30 };
println!("{}", person);     // Alice (30 years old)
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll implement `Display` for `JsonValue` to serialize it back to valid JSON — completing the round-trip from raw text to parsed data and back.


---


## Further Reading


- [The Rust Book — The Display Trait](https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits) — Display vs Debug
- [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) — the Display trait documentation
- [ToString trait](https://doc.rust-lang.org/std/string/trait.ToString.html) — the blanket implementation from Display

