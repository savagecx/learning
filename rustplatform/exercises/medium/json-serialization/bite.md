# JSON Serialization

- Level: medium
- Author: Tanner-Ray-Martin

## Instructions
🎯 In Python, JSON serialization is built in and works with dicts:

```python
import json
data = json.dumps({"host": "localhost", "port": 8080})  # str
config = json.loads(data)                                 # dict
config["host"]  # "localhost" — but what if "host" is missing? Runtime error.
```

Rust takes a different approach: you serialize and deserialize directly to and from **typed structs**. If the JSON doesn't match the struct's shape, you get an error — no silent missing keys, no wrong types hiding in a dict.

### Serde — Rust's serialization framework

[Serde](https://serde.rs/) is Rust's de facto serialization ecosystem. It's not built into the standard library — it's an external crate, but it's used almost universally. Two crates work together:

- **`serde`** — the core framework that defines the `Serialize` and `Deserialize` traits
- **`serde_json`** — the JSON-specific implementation (there are also crates for TOML, YAML, MessagePack, etc.)


### `#[derive()]` — auto-implementing traits

The magic that makes serde ergonomic is `#[derive()]`. Instead of manually writing serialization code, you annotate your struct:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
}
```

`#[derive(Serialize, Deserialize)]` tells the compiler to *generate* the serialization code at compile time. No reflection, no runtime overhead. You've already seen `#[derive(Debug, PartialEq)]` — serde uses the same mechanism.

### Serialization and deserialization

With the traits derived, the actual conversion is one line each:

```rust
// struct → JSON string
let json: String = serde_json::to_string(&config;).unwrap();

// JSON string → struct
let config: Config = serde_json::from_str(&json;).unwrap();
```

Both return `Result` — the operation can fail (invalid JSON, missing fields, wrong types). Here we use `.unwrap()` for simplicity, which panics on error. In production code you'd handle the `Result` properly.
Notice how `from_str` knows to produce a `Config` from the return type — the same type inference that drives `.collect()`.

---


## Your Task


1. Implement `serialize_person(person: &Person) -> String` — convert a `Person` to a JSON string
2. Implement `deserialize_person(json: &str) -> Person` — parse a JSON string into a `Person`

The `Person` struct and its derive macros are already provided in the template.

---


## Example


```rust
let person = Person { name: "Alice".to_string(), age: 30 };
let json = serialize_person(&person;);
assert_eq!(json, "{\"name\":\"Alice\",\"age\":30}");

let parsed = deserialize_person(&json;);
assert_eq!(parsed.name, "Alice");
```


---


## Further Reading


- [Serde documentation](https://serde.rs/) — the official guide with examples for all formats
- [The Rust Book — Derive](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) — how `#[derive()]` works
- [serde_json crate](https://docs.rs/serde_json/latest/serde_json/) — API reference for JSON serialization

