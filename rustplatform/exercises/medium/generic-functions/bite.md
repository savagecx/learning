# Generic Functions

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, functions work with any type by default — duck typing:

```python
def middle(items):
    if not items:
        return None
    return items[len(items) // 2]

middle([10, 20, 30])          # 20
middle(["a", "b", "c", "d"]) # "b"
```

No type annotations needed. If it has `__getitem__`, it works. If it doesn't — runtime error.
Rust's approach is different: generic functions work with many types, but the compiler verifies at compile time that each type supports the operations you use. The `<T>` syntax declares a type parameter:

```rust
fn middle(items: &[T]) -> Option<&T;> {
    if items.is_empty() {
        None
    } else {
        Some(&items;[items.len() / 2])
    }
}
```

This works with `&[i32]`, `&[String]`, `&[bool]` — any slice type. The compiler generates optimized code for each concrete type you use. Zero runtime cost, full type safety.

### When T isn't enough: trait bounds

Some functions need types that support specific operations. A `find_min` function needs to compare values. In Python, `min()` works on anything that supports `<` — and you find out at runtime if it doesn't.
In Rust, you declare what `T` must support using **trait bounds**:

```rust
fn find_min(items: &[T]) -> Option<&T;> {
    items.iter().min()
}
```

`T: Ord` means "T must implement the Ord trait" (total ordering). The compiler checks this at compile time — if you try `find_min` on a type that isn't `Ord`, you get a compile error, not a runtime crash.
Common trait bounds:
| Bound | What it requires | Python equivalent |
|-------|-----------------|-------------------|
| `T: Ord` | Total ordering (`<`, `>`, `==`) | `__lt__` + `__eq__` |
| `T: PartialEq` | Equality comparison (`==`) | `__eq__` |
| `T: Clone` | Can be duplicated | `copy.copy()` |
| `T: Display` | User-facing formatting | `__str__` |
| `T: Debug` | Developer formatting | `__repr__` |

### Multiple bounds with +

When a function needs multiple capabilities:

```rust
fn print_and_clone(item: &T;) -> T {
    println!("{}", item);
    item.clone()
}
```

`T: Display + Clone` means T must implement both traits. Python achieves this through duck typing — if the object has both `__str__` and supports copy, it works. Rust requires you to declare the requirements explicitly.

### Slices: the generic view

Notice that the functions take `&[T]` — a slice reference. This works with vectors, arrays, and other contiguous collections:

```rust
let vec = vec![1, 2, 3];
let arr = [1, 2, 3];

middle(&vec;);    // works — Vec coerces to &[T]
middle(&arr;);    // works — [T; N] coerces to &[T]
```

This is like Python functions accepting any sequence — but with compile-time guarantees about what operations are available.

---


## Your Task


1. `first_and_last<T>` — return `Some((&first, &last))` or `None` if the slice is empty. No trait bounds needed — you're just returning references.
2. `find_max<T: Ord>` — return `Some(&max)` or `None` if empty. The `Ord` bound lets you compare elements.
3. `count_matches<T: PartialEq>` — count how many times `target` appears in the slice. The `PartialEq` bound lets you check equality.


---


## Example


```rust
let nums = vec![1, 2, 3, 4, 5];
assert_eq!(first_and_last(&nums;), Some((&1, &5)));

let words = vec!["apple", "banana", "cherry"];
assert_eq!(find_max(&words;), Some(&"cherry"));

let data = vec![1, 2, 2, 3, 2, 4];
assert_eq!(count_matches(&data;, &2), 3);
assert_eq!(count_matches(&data;, &5), 0);
```


---


## Further Reading


- [The Rust Book — Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html) — generic syntax and usage
- [The Rust Book — Traits as Parameters](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters) — trait bounds on functions
- [std::cmp::Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html) — the total ordering trait

