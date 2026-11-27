# Advanced Pattern Matching

- Level: medium
- Author: Pybites

## Instructions
🎯 Python 3.10 introduced structural pattern matching:

```python
match command:
    case {"action": "move", "x": x, "y": y}:
        print(f"Moving to {x}, {y}")
    case {"action": "quit"}:
        print("Goodbye")
```

Rust's `match` goes further with features you'll use constantly: **ranges**, **multiple patterns**, **guards**, **destructuring**, and **exhaustiveness checking**. This exercise drills all of them.

### Ranges and multiple patterns


```rust
match score {
    90..=100 => "A",                // range pattern
    75..=89 | 70..=74 => "B/C",    // multiple patterns with |
    _ => "below",                   // catch-all
}
```

Python's `match` doesn't support ranges directly — you'd use `case x if 90 <= x <= 100`. Rust makes it first-class syntax.

### Guards

Guards add conditions to patterns:

```rust
match age {
    n if n >= 18 => "adult",
    13..=17 => "teen",
    _ => "child",
}
```

Python has guards too (`case x if x >= 18`), so this concept transfers directly.

### Destructuring

Rust can destructure tuples, structs, and enums in match arms:

```rust
match (width, height) {
    (w, h) if w == h => w * w,       // destructure tuple + guard
    (w, h) => w * h,
}
```


### Enum destructuring

Enums with data are destructured to extract their contents:

```rust
enum Command {
    Say(String),
    Resize { w: u32, h: u32 },
    Exit,
}

match cmd {
    Command::Say(s) if !s.is_empty() => s,
    Command::Say(_) => "(nothing)".into(),
    Command::Resize { w, h } => format!("{w}x{h}"),
    Command::Exit => "done".into(),
}
```

Each variant can have its own guard. The compiler ensures all variants are handled.

---


## Your Task


1. `classify_char(c: char) -> &'static str` — classify a character:
2. `'0'..='9'` → `"digit"`
3. `'a'..='z' | 'A'..='Z' | '_'` → `"alpha"`
4. `'+' | '-' | '*' | '/' | '='` → `"op"`
5. `' ' | '\t' | '\n'` → `"ws"`
6. Anything else → `"other"`
7. `sum_positive_pairs(input: &[(i32, i32)]) -> i32` — sum `x + y` only for pairs where both values are positive, using tuple destructuring with a guard.
8. Define the `Message` enum and implement `route(msg: Message) -> String`:
9. `Ping` → `"pong"`
10. `Echo(s)` → return `s` if non-empty, else `"silence"` (use a guard)
11. `Move { x, y }` → `"noop"` if both zero, else `"move:{x},{y}"`
12. `Quit` → `"bye"`


---


## Example


```rust
assert_eq!(classify_char('8'), "digit");
assert_eq!(classify_char('_'), "alpha");
assert_eq!(classify_char('+'), "op");

assert_eq!(sum_positive_pairs(&[(1, 2), (-1, 3), (4, 0)]), 3);

use Message::*;
assert_eq!(route(Ping), "pong");
assert_eq!(route(Echo("hi".into())), "hi");
assert_eq!(route(Echo("".into())), "silence");
assert_eq!(route(Move { x: 0, y: 0 }), "noop");
assert_eq!(route(Move { x: 2, y: -1 }), "move:2,-1");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), pattern matching is the heart of your JSON parser — matching on token variants, destructuring `String(s)` and `Number(n)`, and using guards to handle edge cases.


---


## Further Reading


- [The Rust Book — Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html) — comprehensive pattern syntax
- [The Rust Book — Match Guards](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#adding-conditionals-with-match-guards) — adding conditions to patterns
- [The Rust Book — Destructuring](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#destructuring-to-break-apart-values) — tuples, structs, and enums

