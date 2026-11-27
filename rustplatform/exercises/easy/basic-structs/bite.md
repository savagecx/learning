# Basic Structs

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you'd use a class or dataclass to group related data:

```python
from dataclasses import dataclass

@dataclass
class City:
    name: str
    country: str
    population: int

    def summary(self) -> str:
        return f"{self.name} ({self.country}) — population: {self.population}"
```

Rust's equivalent is a `struct` with an `impl` block for methods:

```rust
struct City {
    name: String,
    country: String,
    population: u64,
}

impl City {
    fn summary(&self;) -> String {
        format!(
            "{} ({}) — population: {}",
            self.name, self.country, self.population
        )
    }
}
```

Key differences from Python:

- **Data and methods are separate.** The `struct` defines fields. The `impl` block defines methods. In Python, both live inside the `class`.
- **`&self` is explicit.** Python's `self` is always a reference. Rust makes you write `&self` (borrow) vs `self` (consume) — you choose whether the method keeps the struct alive.
- **Fields are private by default.** Rust uses `pub` to opt-in to visibility (covered in a later exercise). Python uses `_underscore` conventions.


### Creating struct instances

Rust doesn't have constructors like Python's `__init__`. You create instances directly:

```rust
let city = City {
    name: String::from("Tokyo"),
    country: String::from("Japan"),
    population: 14_000_000,
};
```

Or you can define a `new` function by convention (not a language feature):

```rust
impl City {
    fn new(name: &str;, country: &str;, population: u64) -> Self {
        City {
            name: name.to_string(),
            country: country.to_string(),
            population,  // shorthand when field name matches variable name
        }
    }
}
```


---


## Your Task


1. Define a `Book` struct with fields: `title: String`, `author: String`, `pages: u32`
2. Implement a `describe(&self) -> String` method that returns: `"The book '{title}' by {author} has {pages} pages."`


---


## Example


```rust
let book = Book {
    title: String::from("The Hobbit"),
    author: String::from("J.R.R. Tolkien"),
    pages: 310,
};
assert_eq!(
    book.describe(),
    "The book 'The Hobbit' by J.R.R. Tolkien has 310 pages."
);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll refactor free functions into `Tokenizer` and `JsonParser` structs — encapsulating state with methods like `advance()`, `peek()`, and `parse_value()`.


---


## Further Reading


- [The Rust Book — Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) — struct syntax
- [The Rust Book — Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) — impl blocks and &self

