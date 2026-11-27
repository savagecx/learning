# Generic Structs

- Level: medium
- Author: Pybites

## Instructions
🎯 Python's classes can hold any type without declaring it:

```python
class Container:
    def __init__(self, item):
        self.item = item

Container(42)
Container("hello")
Container([1, 2, 3])
```

No type declarations needed — `item` can be anything. With type hints, you can be more specific:

```python
from typing import Generic, TypeVar
T = TypeVar("T")

class Container(Generic[T]):
    def __init__(self, item: T):
        self.item = item
```

But Python's generics are only for type checkers. At runtime, `Container[int]` and `Container[str]` are the same class.
Rust's generic structs are different — they're real, distinct types at compile time:

```rust
struct Container {
    item: T,
}

// Container and Container are different types
// The compiler generates optimized code for each
```


### Defining generic structs

The `<T>` after the struct name declares a type parameter. Use it in fields:

```rust
struct Container {
    item: T,
}

struct Entry {
    key: K,
    val: V,
}
```

`Container<T>` has one type parameter — both occurrences must be the same type. `Entry<K, V>` has two — `key` and `val` can be different types.

### Implementing methods with impl

To add methods, you repeat the type parameter on `impl`:

```rust
impl Container {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn unwrap(self) -> T {
        self.item
    }
}
```

The `impl<T>` says "this implementation works for any T." `Self` is shorthand for `Container<T>` — the concrete type being implemented.

### self vs &self: consuming vs borrowing

Notice `unwrap` takes `self` (not `&self`). This means it **consumes** the container:

```rust
let c = Container::new(42);
let item = c.unwrap();  // c is consumed, item is 42
// c can't be used anymore — it was moved
```

In Python, every method gets `self` as a reference — the object continues to exist. In Rust, `self` (without `&`) takes ownership, destroying the original. This is the "into" naming convention: consuming the struct to extract the inner value.
Compare:
- `&self` — borrows, original survives (like Python's `self`)
- `self` — consumes, original is gone (no Python equivalent)

### Swapping type parameters

A method on `Entry<K, V>` can return `Entry<V, K>` — the type parameters reversed:

```rust
impl Entry {
    fn flip(self) -> Entry {
        Entry {
            key: self.val,
            val: self.key,
        }
    }
}
```

The return type `Entry<V, K>` is a different concrete type than `Entry<K, V>`. The compiler tracks this — after flipping, the type system knows `key` is now type `V` and `val` is type `K`.

### You've already used generic structs

`Vec<T>`, `Option<T>`, `Result<T, E>`, `HashMap<K, V>` — Rust's most important types are all generic structs. Building your own follows the same pattern.

---


## Your Task


1. Create `Wrapper<T>` with a private field `value: T`:
2. `new(value: T) -> Self` — creates a new wrapper
3. `into_inner(self) -> T` — consumes the wrapper, returns the value
4. Create `Pair<T, U>` with public fields `first: T` and `second: U`:
5. `new(first: T, second: U) -> Self` — creates a new pair
6. `swap(self) -> Pair<U, T>` — consumes the pair, returns a new one with types swapped


---


## Example


```rust
let w = Wrapper::new(42);
assert_eq!(w.into_inner(), 42);

let w = Wrapper::new(String::from("hello"));
assert_eq!(w.into_inner(), "hello");

let p = Pair::new("hello", 123);
assert_eq!(p.first, "hello");
let swapped = p.swap();
assert_eq!(swapped.first, 123);
assert_eq!(swapped.second, "hello");
```


---


## Further Reading


- [The Rust Book — Generic Data Types in Structs](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions) — generic struct syntax
- [The Rust Book — Generic Data Types in Method Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions) — impl and methods
- [The Rust Book — Ownership and Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) — self vs &self vs &mut self

