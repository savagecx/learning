# Re-exports and API Design

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, a well-designed library lets users import from the top level:

```python
from mylib import User, Config, AppError
```

Even though internally `User` might live in `mylib.internal.models.user`. You achieve this by importing into `__init__.py`:

```python
# mylib/__init__.py
from .internal.models.user import User
from .internal.config import Config
from .internal.errors import AppError
```

Rust uses the same pattern with `pub use`. You organize code internally however makes sense, then create a clean public API with re-exports. The internal structure is hidden — users never see it.

### The prelude pattern

Many Rust libraries provide a `prelude` module — a single import that brings all the commonly-used types into scope:

```rust
// In your library
pub mod prelude {
    pub use crate::Config;
    pub use crate::Error;
    pub use crate::Client;
}

// Users write:
use my_lib::prelude::*;
```

This is like Python's "import everything you need" idiom, but organized into a dedicated module. The `prelude` convention signals: "these are the essentials — import all of them."
Standard library examples: `std::io::prelude::*`, `std::prelude::*` (auto-imported). Popular crates like `tokio`, `serde`, and `diesel` all provide preludes.

### Flattening deep module paths

Without re-exports, users navigate your internal structure:

```rust
// Without re-exports — exposes internals
use my_lib::internal::types::User;
use my_lib::internal::errors::ValidationError;
```

With `pub use` at the crate root:

```rust
// With re-exports — clean API
use my_lib::User;
use my_lib::ValidationError;
```

The internal structure can change freely without breaking callers. This is the same benefit Python's `__init__.py` provides, but with compiler-enforced privacy — if `internal` isn't `pub`, users literally can't reach into it.

### Combining re-exports with validation

A common pattern: keep the struct's `new()` constructor in an internal module (where it can access private fields) and re-export the struct at the top level. Users interact with the clean API while the implementation details stay hidden:

```rust
mod internal {
    pub mod inventory {
        pub struct Product { pub name: String, pub price: f64 }

        impl Product {
            pub fn new(name: &str;, price: f64) -> Result {
                // validation logic here
            }
        }
    }
}

pub use internal::inventory::Product;  // users see: my_lib::Product
```


### Cross-module imports with `super::`

When `types` needs to use `errors` from a sibling module, it reaches through the parent:

```rust
mod internal {
    pub mod inventory {
        use super::super::internal::warnings::InvalidData;
        // or simply: use crate::internal::warnings::InvalidData;
    }
    pub mod warnings { /* ... */ }
}
```

Using `crate::` (absolute path) is often clearer than chaining `super::` when the path gets deep.

---


## Your Task

Build a mini library structure with clean API design:

1. Create an `internal` module with:
2. `types` submodule containing `pub struct User { pub name: String, pub age: u32 }`
3. `errors` submodule containing `#[derive(Debug)] pub struct ValidationError(pub String)`
4. Implement `User::new(name: &str, age: u32) -> Result<User, ValidationError>`:
5. Return error if name is empty
6. Return error if age > 150
7. Create a `prelude` module that re-exports `User` and `ValidationError`
8. At the crate root, re-export `User` directly with `pub use`


---


## Example


```rust
// Via prelude
use crate::prelude::*;
let user = User::new("Alice", 30).unwrap();
assert_eq!(user.name, "Alice");

// Direct access via crate root re-export
let user = crate::User::new("Bob", 25).unwrap();

// Validation errors
assert!(User::new("", 30).is_err());
assert!(User::new("Alice", 200).is_err());
```


---


## Further Reading


- [The Rust Book — Re-exporting with pub use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use) — API design with re-exports
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) — conventions for public APIs
- [The Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html) — how the standard library uses the prelude pattern

