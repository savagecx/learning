# Option Combinators

- Level: easy
- Author: Pybites

## Instructions
In the previous exercise, you used `match` to handle `Option`. But matching every time gets verbose:

```rust
let name: Option = get_name();
let upper = match name {
    Some(n) => Some(n.to_uppercase()),
    None => None,
};
```

Rust provides **combinators** — methods that transform Options without explicit matching.

### `.map()` — transform the inner value


```rust
let name: Option = Some("alice".to_string());
let upper = name.map(|n| n.to_uppercase());
// Some("ALICE")

let empty: Option = None;
let upper = empty.map(|n| n.to_uppercase());
// None — closure never runs
```

In Python, this is like: `x.upper() if x is not None else None`

### `.unwrap_or()` — provide a default


```rust
let count: Option = None;
let value = count.unwrap_or(0);  // 0

let count: Option = Some(42);
let value = count.unwrap_or(0);  // 42
```


### `.unwrap_or_else()` — lazy default

When computing the default is expensive, use a closure:

```rust
let value = count.unwrap_or_else(|| expensive_computation());
```


### `.and_then()` — chain Option-returning functions

When your transformation also returns an Option:

```rust
fn find_name(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Alice"),
        2 => Some("Bob"),
        _ => None,
    }
}

let id: Option = Some(1);
let name: Option<&str;> = id.and_then(find_name);
// Some("Alice")
```

`.map()` would give `Option<Option<&str>>`. `.and_then()` flattens it.

### `.filter()` — keep only if condition holds


```rust
let age: Option = Some(25);
let adult = age.filter(|&a;| a >= 18);  // Some(25)

let age: Option = Some(15);
let adult = age.filter(|&a;| a >= 18);  // None
```


---


## Your Task


1. `double_if_some(n: Option<i32>) -> Option<i32>` — double the value if present
2. `get_or_default(opt: Option<String>, default: &str) -> String` — return value or default
3. `first_even(numbers: &[i32]) -> Option<i32>` — find first even, then double it (chain `.find()` with `.map()`)
4. `parse_and_add(s: Option<&str>, add: i32) -> Option<i32>` — parse string to i32 and add (use `.and_then()`)


---


## Example


```rust
assert_eq!(double_if_some(Some(5)), Some(10));
assert_eq!(double_if_some(None), None);

assert_eq!(get_or_default(Some("hello".to_string()), "default"), "hello");
assert_eq!(get_or_default(None, "default"), "default");

assert_eq!(first_even(&[1, 3, 4, 5]), Some(8));  // 4 doubled
assert_eq!(first_even(&[1, 3, 5]), None);

assert_eq!(parse_and_add(Some("10"), 5), Some(15));
assert_eq!(parse_and_add(Some("abc"), 5), None);
assert_eq!(parse_and_add(None, 5), None);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll chain Option combinators to safely navigate nested JSON — like `value.get("user").and_then(|v| v.get("name")).and_then(|v| v.as_str())`.


---


## Further Reading


- [Option in std](https://doc.rust-lang.org/std/option/enum.Option.html) — all combinator methods
- [Rust By Example — Combinators](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html) — practical examples

