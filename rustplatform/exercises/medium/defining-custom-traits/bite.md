# Defining Custom Traits

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, you get polymorphism through duck typing and abstract base classes:

```python
from abc import ABC, abstractmethod

class Describable(ABC):
    @abstractmethod
    def describe(self) -> str:
        ...

    @abstractmethod
    def tag(self) -> str:
        ...

class Book(Describable):
    def __init__(self, title, author):
        self.title = title
        self.author = author

    def describe(self) -> str:
        return f"{self.title} by {self.author}"

    def tag(self) -> str:
        return "book"
```

This is class-based inheritance: `Book` *is a* `Describable`. In Python, the ABC enforces that subclasses implement the abstract methods. Without ABC, duck typing handles it — if it has `describe()`, it's describable.
Rust uses **traits** instead of inheritance. A trait defines behavior that types can implement:

```rust
trait Describable {
    fn describe(&self;) -> String;
    fn tag(&self;) -> &str;
}
```

The key difference: traits are *implemented for* types, not *inherited by* them. Any type can implement any trait — no class hierarchy needed. `Book` implements `Describable` not because it inherits from it, but because it provides the required methods.

### Implementing a trait


```rust
impl Describable for Book {
    fn describe(&self;) -> String {
        format!("{} by {}", self.title, self.author)
    }

    fn tag(&self;) -> &str; {
        "book"
    }
}
```

`impl Trait for Type` — you specify which trait is being implemented for which type. Each required method must be provided. The compiler checks this — forget a method and you get a compile error, not a runtime `NotImplementedError`.

### Traits vs Python ABCs

Both define "contracts" that types must fulfill. The differences:

- **No inheritance required.** In Python, you `class Book(Describable)`. In Rust, `impl Describable for Book` — Book doesn't inherit from anything.
- **Implement for external types.** You can implement your trait for types from other libraries. In Python, you can't make `int` inherit from your ABC (without monkey-patching).
- **Compile-time checking.** Missing a trait method is a compile error. Python's ABC raises `TypeError` at instantiation time.


### Default methods

Traits can provide default implementations that types can optionally override:

```rust
trait Summary {
    fn summarize(&self;) -> String;  // required

    fn preview(&self;) -> String {   // default — can be overridden
        format!("Read more: {}", self.summarize())
    }
}
```

This is like providing a method body in an ABC:

```python
class Summary(ABC):
    @abstractmethod
    def summarize(self) -> str: ...

    def preview(self) -> str:  # default implementation
        return f"Read more: {self.summarize()}"
```


### Trait objects: mixed collections

Python lists can hold any type. In Rust, `Vec<T>` requires a single type. To hold different types that share a trait, use **trait objects** with `dyn`:

```rust
let items: Vec> = vec![
    Box::new(Book { title: "Dune".into(), author: "Herbert".into() }),
    Box::new(Movie { title: "Arrival".into(), year: 2016 }),
];

for item in &items; {
    println!("[{}] {}", item.tag(), item.describe());
}
```

`Box<dyn Describable>` means "a heap-allocated value that implements Describable." This is dynamic dispatch — like Python's polymorphism, with a small runtime cost for the indirection.

---


## Your Task


1. Define a `Drawable` trait with two required methods:
2. `fn draw(&self) -> String`
3. `fn bounding_box(&self) -> (u32, u32)` — returns (width, height)
4. Implement `Drawable` for `Circle`:
5. `draw` returns `"Circle(radius=X)"` where X is the radius
6. `bounding_box` returns `(2*radius, 2*radius)`
7. Implement `Drawable` for `Rectangle`:
8. `draw` returns `"Rectangle(WxH)"` where W is width and H is height
9. `bounding_box` returns `(width, height)`


---


## Example


```rust
let circle = Circle { radius: 5 };
assert_eq!(circle.draw(), "Circle(radius=5)");
assert_eq!(circle.bounding_box(), (10, 10));

let rect = Rectangle { width: 4, height: 3 };
assert_eq!(rect.draw(), "Rectangle(4x3)");
assert_eq!(rect.bounding_box(), (4, 3));

// Trait objects — different types in one collection
let shapes: Vec> = vec![
    Box::new(Circle { radius: 2 }),
    Box::new(Rectangle { width: 3, height: 4 }),
];
assert_eq!(shapes[0].draw(), "Circle(radius=2)");
```


---


## Further Reading


- [The Rust Book — Defining a Trait](https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait) — trait syntax and implementation
- [The Rust Book — Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) — dyn Trait and dynamic dispatch
- [The Rust Book — Default Implementations](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) — traits with default methods

