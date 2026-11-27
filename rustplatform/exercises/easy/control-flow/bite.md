# Control Flow

- Level: easy
- Author: Pybites

## Instructions
🎯 Python's `if/elif/else` and `match` (3.10+) handle branching:

```python
def describe_temp(celsius):
    if celsius >= 30:
        label = "Hot"
    elif celsius >= 15:
        label = "Mild"
    else:
        label = "Cold"

    match label[0]:
        case "H": return "stay inside"
        case "M": return "enjoy"
        case _:   return "bundle up"
```

Rust has the same constructs, but with a key difference: **`if` and `match` are expressions.** They return values:

```rust
let label = if celsius >= 30 {
    "Hot"
} else if celsius >= 15 {
    "Mild"
} else {
    "Cold"
};
```

No ternary operator needed — `if` itself returns a value. Python's equivalent would be `label = "Hot" if celsius >= 30 else ...`, but Rust's version scales to any number of branches.

### match: exhaustive pattern matching

Rust's `match` is more powerful than Python's structural `match`. The compiler **requires** you to handle every possible case:

```rust
match label.chars().next() {
    Some('H') => "stay inside".to_string(),
    Some('M') => "enjoy".to_string(),
    _ => "bundle up".to_string(),
}
```

The `_` wildcard catches everything else. If you forget a case, the compiler tells you — no silent fall-through, no missed branches.

---


## Your Task

Implement `grade_message(score: i32) -> String`:

1. Use `if/else if/else` to determine the grade:
2. `score >= 90` → `"Excellent"`
3. `score >= 75` → `"Good"`
4. `score >= 50` → `"Pass"`
5. Otherwise → `"Fail"`
6. Use `match` on the first character of the grade to return:
7. `'E'` → `"Top"`
8. `'G'` → `"Decent"`
9. `'P'` → `"Basic"`
10. Anything else → `"None"`


---


## Example


```rust
assert_eq!(grade_message(95), "Top");
assert_eq!(grade_message(78), "Decent");
assert_eq!(grade_message(52), "Basic");
assert_eq!(grade_message(10), "None");
```


---


## Further Reading


- [The Rust Book — Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — if expressions, loops
- [The Rust Book — The match Control Flow](https://doc.rust-lang.org/book/ch06-02-match.html) — exhaustive pattern matching

