# Mini Parser

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, parsing a simple value might use `json.loads` or manual checking:

```python
import json

def parse_primitive(text):
    text = text.strip()
    if text in ("true", "false"):
        return text == "true"
    if text == "null":
        return None
    return int(text)  # raises ValueError on bad input
```

Simple, but error handling is implicit — `int()` raises, `json.loads` raises, and callers need to know what exceptions to catch.
This exercise combines everything from the intro track: **enums with data**, **pattern matching**, **Result for errors**, **string processing**, and **character-level parsing**. You'll build a parser that handles JSON-style primitives with explicit, type-safe error handling.

### Enums as both values and errors

You'll define two enums — one for successfully parsed values, one for error cases:

```rust
enum JsonValue {
    Bool(bool),
    Null,
    Number(i64),
}

enum ParseError {
    Empty,
    Invalid,
    Overflow,
}
```

The return type `Result<JsonValue, ParseError>` makes every possible outcome explicit. The caller sees exactly what can succeed and what can fail — no hidden exceptions.

### Parsing strategy

Instead of regex, you'll handle each value type explicitly:

1. Trim whitespace
2. Check for keyword literals (`true`, `false`, `null`)
3. Parse integers (Rust's `parse::<i64>()` handles optional `+`/`-` signs)
4. Use the error's `kind()` to distinguish overflow from invalid input


---


## Your Task

Define the `JsonValue` and `ParseError` enums (already provided), then implement:
`parse_primitive(input: &str) -> Result<JsonValue, ParseError>`
Rules:
- Trim whitespace; if empty → `Err(Empty)`
- `"true"` → `Ok(Bool(true))`, `"false"` → `Ok(Bool(false))`, `"null"` → `Ok(Null)`
- Otherwise parse as integer with optional `+`/`-` prefix
- No digits after sign → `Invalid`
- Extra non-whitespace after the value → `Invalid`
- Integer doesn't fit in `i64` → `Overflow`

---


## Example


```rust
use JsonValue::*;
use ParseError::*;

assert_eq!(parse_primitive("true"), Ok(Bool(true)));
assert_eq!(parse_primitive(" false "), Ok(Bool(false)));
assert_eq!(parse_primitive("null"), Ok(Null));

assert_eq!(parse_primitive("42"), Ok(Number(42)));
assert_eq!(parse_primitive("-7"), Ok(Number(-7)));
assert_eq!(parse_primitive("+9"), Ok(Number(9)));

assert_eq!(parse_primitive(""), Err(Empty));
assert_eq!(parse_primitive("12x"), Err(Invalid));
assert_eq!(parse_primitive("+"), Err(Invalid));
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll build a full recursive-descent JSON parser on top of patterns like this — parsing arrays, objects, and nested values into a `JsonValue` tree.


---


## Further Reading


- [The Rust Book — Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — enums with data
- [The Rust Book — Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) — Result-based error handling
- [Crafting Interpreters — Scanning](https://craftinginterpreters.com/scanning.html) — character-level parsing techniques

