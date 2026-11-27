# Option Handling

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, any function can return `None` — and nothing in the language forces you to check for it:

```python
def find_user(user_id):
    return db.get(user_id)  # might return None

user = find_user(42)
print(user.name)  # AttributeError at runtime if None
```

The famous "billion-dollar mistake" — `None` sneaks through silently until something crashes. Rust eliminates this with **`Option<T>`**.

### Option: Rust's "maybe" type

`Option<T>` is an enum with two variants:

```rust
enum Option {
    Some(T),  // a value is present
    None,     // no value
}
```

When a function might not have a value to return, it returns `Option<T>` instead of a bare `T`. The compiler then **forces** you to handle both cases before you can use the value:

```rust
let maybe: Option = Some(42);

// Can't just use maybe as an i32 — must unwrap it first
let value = match maybe {
    Some(n) => n,
    None => 0,  // provide a default
};
```

This isn't optional (no pun intended) — the compiler rejects code that ignores the `None` case. No more surprise `AttributeError`s.

### Building your own Options

Returning `Option` from your functions is straightforward — `Some(value)` and `None` are constructors:

```rust
fn find_positive(n: i32) -> Option {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}
```


### Option combinators: `.map()`

Instead of matching every time, you can chain operations on Options with **combinators**. `.map()` transforms the inner value while keeping the `Option` wrapper:

```rust
let name: Option = Some("alice".to_string());
let upper: Option = name.map(|n| n.to_uppercase());
// Some("ALICE")

let empty: Option = None;
let upper: Option = empty.map(|n| n.to_uppercase());
// None — the closure never runs
```

This is like Python's pattern of `x.upper() if x is not None else None` — but cleaner. If the value is `Some`, the closure runs. If it's `None`, it stays `None`. No `if` needed.

### References and `.copied()`

When searching through a slice with `.iter().find()`, the result is `Option<&T>` — a reference wrapped in an Option. If you need `Option<T>` (an owned value), use `.copied()` to convert:

```rust
let numbers = [1, 2, 3, 4];
let found: Option<&i32;> = numbers.iter().find(|&&x;| x > 2);
let owned: Option = found.copied();  // Option<&i32;> -> Option
```

In Python, `next()` on a generator gives you the value directly. In Rust, iterators over collections yield references, so you often need `.copied()` (for `Copy` types) or `.cloned()` to get an owned value out.

---


## Your Task

Implement three functions:

1. `find_first_even(numbers: &[i32]) -> Option<i32>` — return the first even number in the slice, or `None` if there isn't one
2. `safe_divide(a: i32, b: i32) -> Option<i32>` — return `Some(a / b)`, or `None` if `b` is zero
3. `get_username(user: Option<User>) -> Option<String>` — extract the `name` field from an optional `User`


---


## Example


```rust
assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
assert_eq!(find_first_even(&[1, 3, 5]), None);

assert_eq!(safe_divide(10, 2), Some(5));
assert_eq!(safe_divide(10, 0), None);

let user = Some(User { name: "Alice".to_string() });
assert_eq!(get_username(user), Some("Alice".to_string()));
assert_eq!(get_username(None), None);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), your `JsonValue` type will have safe accessors like `.as_str()` and `.as_f64()` that return `Option` — no more runtime type errors when accessing parsed data.


---


## Further Reading


- [The Rust Book — The Option Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) — why Rust has no null
- [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html) — full API with all combinators (`.map()`, `.and_then()`, `.unwrap_or()`, etc.)
- [Iterator::find](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) — returns `Option<&T>` for the first match

