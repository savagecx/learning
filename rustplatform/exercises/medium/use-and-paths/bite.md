# Use and Paths

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, you bring names into scope with `import` and `from ... import`:

```python
import math
from collections import Counter, defaultdict
from os.path import join as path_join
```

Rust's `use` statement works the same way — it brings items from other modules into the current scope so you don't have to write full paths everywhere.

### Path types

Rust has three ways to reference items in the module tree:
**`crate::`** — absolute path from the crate root (like Python's absolute imports):

```rust
use crate::utils::helpers::format_name;
```

**`super::`** — relative path to the parent module (like Python's `from .. import`):

```rust
mod child {
    use super::parent_function;  // import from parent module
}
```

**`self::`** — relative path from the current module (rarely needed, explicit):

```rust
use self::submodule::helper;
```

In Python, absolute imports (`from mypackage.module import func`) and relative imports (`from ..module import func`) serve the same purpose. Rust's `crate::` and `super::` are the equivalents.

### Use patterns

Rust supports several import styles, all familiar from Python:

```rust
// Single item (like: from collections import HashMap)
use std::collections::HashMap;

// Multiple items (like: from collections import Counter, defaultdict)
use std::collections::{HashMap, HashSet};

// Glob import (like: from collections import *)
use std::collections::*;  // use sparingly

// Rename (like: from os.path import join as path_join)
use std::fmt::Result as FmtResult;
```

The grouped import `{HashMap, HashSet}` is especially handy — it avoids repeating the path prefix. Python's equivalent is separate `from` lines.

### `pub use` — re-exporting

This is something Python doesn't have a direct equivalent for. `pub use` imports an item *and* makes it available to users of your module:

```rust
mod internal {
    pub fn helper() -> i32 { 42 }
}

pub use internal::helper;  // users can now call crate::helper() directly
```

This is like defining `helper` in Python's `__init__.py` — you're creating a public-facing API that's organized differently from the internal structure. Users see a clean API; you keep the implementation organized however makes sense internally.
`pub use` is common in Rust libraries: the internal module tree might be deep and organized by implementation concerns, but `pub use` creates a flat, convenient public API.

### `super::` for cross-module imports

When one module needs types from a sibling module, it reaches through the parent with `super::`:

```rust
mod models {
    pub struct User { pub name: String }
}

mod services {
    use super::models::User;  // import sibling's type through parent

    pub fn greet(user: &User;) -> String {
        format!("Hello, {}!", user.name)
    }
}
```

`super` means "go up one level in the module tree." In the sibling-module pattern above, `use super::models::User` is like Python's `from .models import User` — reaching a sibling through the parent.

---


## Your Task


1. Create a `shapes` module with `Circle` (field: `pub radius: f64`) and `Rectangle` (fields: `pub width: f64`, `pub height: f64`)
2. Create a `calc` module that:
3. Uses `use super::shapes::{Circle, Rectangle};` to import the shapes
4. Implements `pub fn circle_area(c: &Circle) -> f64`
5. Implements `pub fn rect_area(r: &Rectangle) -> f64`
6. At the crate root, use `pub use` to re-export `calc::circle_area` and `calc::rect_area` directly


---


## Example


```rust
let c = shapes::Circle { radius: 2.0 };
assert!((circle_area(&c;) - 12.566).abs() < 0.01);  // via re-export

let r = shapes::Rectangle { width: 3.0, height: 4.0 };
assert_eq!(rect_area(&r;), 12.0);  // via re-export
```


---


## Further Reading


- [The Rust Book — Bringing Paths into Scope with use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html) — `use` syntax and patterns
- [The Rust Book — Paths for Referring to an Item](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) — `crate::`, `super::`, `self::`
- [The Rust Book — Re-exporting with pub use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use) — creating a public API

