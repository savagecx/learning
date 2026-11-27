# Implementing Default

- Level: easy
- Author: Pybites

## Instructions
🎯 In Python, you set default values directly in `__init__`:

```python
class AppSettings:
    def __init__(self, name="myapp", debug=False, timeout=30):
        self.name = name
        self.debug = debug
        self.timeout = timeout

settings = AppSettings()              # all defaults
custom = AppSettings(debug=True)      # override one, keep rest
```

Simple and flexible. Override what you want, keep defaults for the rest.
Rust's equivalent is the `Default` trait. It provides a `default()` method that creates an instance with default values:

```rust
let settings = AppSettings::default();
```


### Deriving Default

When zero/empty values are fine, derive it:

```rust
#[derive(Default)]
struct Coordinate {
    x: f64,    // defaults to 0.0
    y: f64,    // defaults to 0.0
}

let origin = Coordinate::default();
assert_eq!(origin.x, 0.0);
```

Derived `Default` uses each field type's default: `0` for numbers, `false` for `bool`, `""` for `String`, `None` for `Option<T>`. This is like Python's `@dataclass` with `field(default=0)`, except the defaults come from the type system rather than explicit values.

### Manual Default for meaningful values

Zero isn't always the right default. An app should probably default to a meaningful name, not an empty string:

```rust
impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            name: String::from("myapp"),
            debug: false,
            timeout_secs: 30,
        }
    }
}
```

The `default()` method returns `Self` — a fully constructed instance. This is like Python's `__init__` with keyword defaults, but expressed as a separate trait rather than constructor parameters.

### Struct update syntax: Rust's version of keyword defaults

Python lets you override individual keyword arguments. Rust has something similar — the **struct update syntax** with `..`:

```rust
let custom = AppSettings {
    debug: true,
    ..Default::default()  // fill the rest with defaults
};
// name = "myapp", debug = true, timeout_secs = 30
```

The `..Default::default()` fills in any fields you didn't specify. It reads like "start with the default, but change debug to true." This is the closest Rust gets to Python's keyword-argument overrides.
You can also use it with any existing instance:

```rust
let base = AppSettings::default();
let custom = AppSettings {
    debug: true,
    ..base  // fill rest from `base` instead of default
};
```

Note: `..base` moves `base`'s fields, so `base` can't be used afterward (unless all moved fields are `Copy`).

### Where Default shows up

`Default` integrates with the standard library in useful ways:

- `Option::unwrap_or_default()` — returns the default if `None`
- `HashMap::entry().or_default()` — inserts default if key missing
- Generic code can require `T: Default` to create instances without knowing the concrete type


---


## Your Task


1. Derive `Default` for `Counter` (zero-value defaults are fine for both fields)
2. Manually implement `Default` for `ServerConfig` with sensible defaults:
3. `host`: `"localhost"`
4. `port`: `8080`
5. `max_connections`: `100`

Both should work with struct update syntax.

---


## Example


```rust
let counter = Counter::default();
assert_eq!(counter.value, 0);
assert_eq!(counter.step, 0);

let config = ServerConfig::default();
assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);

// Struct update syntax — override one field, keep defaults for rest
let custom = ServerConfig {
    port: 3000,
    ..Default::default()
};
assert_eq!(custom.port, 3000);
assert_eq!(custom.host, "localhost");
```


---


## Further Reading


- [The Rust Book — Default Implementations](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations) — traits with default methods
- [std::default::Default](https://doc.rust-lang.org/std/default/trait.Default.html) — the Default trait documentation
- [Struct Update Syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) — the `..` syntax

