# Visibility Rules

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, encapsulation is by convention. You use a leading underscore to signal "private," but anyone can still access it:

```python
class Sensor:
    def __init__(self, name, value):
        self.name = name
        self._value = value  # "private" — but not really

    @property
    def value(self):
        return self._value

s = Sensor("temp", 72)
s._value = -999  # oops — nothing stops this
```

Python's `@property` gives you a getter, but there's no enforcement. A determined caller can always reach in and modify `_value` directly.
Rust enforces privacy at compile time. If a field isn't `pub`, code outside the module literally cannot access it — the compiler rejects it.

### Struct field visibility

In Rust, struct fields have their own visibility, independent of the struct:

```rust
pub struct Config {
    pub name: String,       // accessible from outside
    secret_key: String,     // private — only this module can see it
}
```

A struct can be `pub` (visible) while having private fields (hidden internals). Outside code can see the struct type but can't construct it directly or read private fields — they must use methods:

```rust
impl Config {
    pub fn new(name: &str;) -> Self {
        Config {
            name: name.to_string(),
            secret_key: generate_key(),  // set internally
        }
    }
}
```

This is like Python's `@property` pattern but enforced. You can't bypass the API.

### Why private fields matter

Private fields let you enforce **invariants** — rules that must always hold. For a thermostat, "temperature must stay within min/max bounds" is an invariant. With public fields, any code could set `temp = 9999`. With private fields and methods:

```rust
pub fn set_temp(&mut; self, temp: f64) -> bool {
    if temp >= self.min && temp <= self.max {
        self.current = temp;
        true
    } else {
        false  // out of range
    }
}
```

The only way to change the temperature is through controlled methods that enforce the bounds. The compiler guarantees this — not a code review, not a convention, the compiler.

### Visibility levels beyond `pub`

Rust has finer-grained visibility than just public/private:
| Syntax | Visible to |
|--------|-----------|
| (no keyword) | Current module only |
| `pub` | Everywhere |
| `pub(crate)` | Current crate only (not exposed to users of your library) |
| `pub(super)` | Parent module |
`pub(crate)` is particularly useful for library code: internal functions that other modules in your crate need, but that shouldn't be part of the public API. Python has no equivalent — `__all__` controls what `from module import *` exposes, but doesn't prevent direct access.

### Constructors and private fields

When a struct has any private fields, it can't be constructed outside the module using struct literal syntax:

```rust
// Outside the module:
let config = Config { name: "app".to_string(), secret_key: "key".to_string() };
// ERROR: secret_key is private
```

You must provide a constructor (like `new()`). This is Rust's way of ensuring that all invariants are established at creation time.

---


## Your Task

Create a `bank` module with:

1. An `Account` struct with:
2. `pub id: u32` — public, anyone can read it
3. `balance: i64` — private, in cents, only accessible through methods
4. Methods on `Account`:
5. `pub fn new(id: u32, initial_balance: i64) -> Self`
6. `pub fn balance(&self) -> i64` — getter for the private field
7. `pub fn deposit(&mut self, amount: i64)` — add to balance
8. `pub fn withdraw(&mut self, amount: i64) -> bool` — subtract from balance, return `false` if insufficient funds (balance unchanged)


---


## Example


```rust
let mut acc = bank::Account::new(1, 1000);
assert_eq!(acc.id, 1);
assert_eq!(acc.balance(), 1000);

acc.deposit(500);
assert_eq!(acc.balance(), 1500);

assert!(acc.withdraw(200));
assert_eq!(acc.balance(), 1300);

assert!(!acc.withdraw(2000));  // fails, balance unchanged
assert_eq!(acc.balance(), 1300);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), your parser exposes `pub fn parse_json()` while keeping internal helpers like `advance()` and `peek()` private — clean API, hidden implementation.


---


## Further Reading


- [The Rust Book — Defining Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) — visibility and the `pub` keyword
- [The Rust Reference — Visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) — `pub(crate)`, `pub(super)`, and other modifiers

