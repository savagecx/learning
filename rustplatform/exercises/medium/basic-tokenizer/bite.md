# Basic Tokenizer

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, a simple tokenizer might use regex or string methods:

```python
import re

def tokenize(text):
    tokens = []
    for match in re.finditer(r'\w+|\d+|[+\-*/(),=]', text):
        tokens.append(match.group())
    return tokens
```

This exercise builds a tokenizer (lexer) in Rust — the first stage of any parser. You'll turn a string like `"sum(x, 42)"` into a stream of typed tokens. This is a step up in complexity: you'll combine enums, pattern matching, string iteration, and character classification.

### Enums with data

Unlike Python's simple `Enum`, Rust enums can carry data in their variants:

```rust
enum Token {
    Ident(String),    // carries the identifier name
    Number(i64),      // carries the numeric value
    Plus,             // no data — just a marker
    Unknown(char),    // carries the unrecognized character
}
```

This is like Python's tagged unions — each variant can hold different data. Python would typically use a dataclass or tuple for this: `("ident", "sum")` or `("number", 42)`.

### The scanning pattern

The tokenizer follows a common pattern: maintain an index, inspect the current character, consume characters while a condition holds, emit a token:

```
mark start → advance while condition → slice start..i → push token
```

Character methods like `c.is_whitespace()`, `c.is_ascii_digit()`, and `c.is_ascii_alphabetic()` replace Python's `str.isspace()`, `str.isdigit()`, and `str.isalpha()`.

---


## Your Task

Given the `Token` enum (already defined):

```rust
enum Token {
    Ident(String),
    Number(i64),
    LParen, RParen,
    Plus, Minus, Star, Slash,
    Comma, Equal,
    Unknown(char),
}
```

Implement `tokenize(input: &str) -> Vec<Token>`:

- **Skip** whitespace
- **Group digits** into one `Number` token
- **Group identifier chars** into one `Ident` (starts with letter or `_`, continues with letters/digits/`_`)
- **Map** single-char operators with `match`
- Emit `Unknown(c)` for anything else


---


## Example


```rust
let toks = tokenize("sum(x, 42) - y3");
assert_eq!(
    toks,
    vec![
        Ident("sum".into()), LParen, Ident("x".into()), Comma,
        Number(42), RParen, Minus, Ident("y3".into())
    ]
);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll extend this tokenizer pattern to handle full JSON — strings with escape sequences, nested structures, and precise error reporting.


---


## Further Reading


- [The Rust Book — Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) — enums with data
- [char methods](https://doc.rust-lang.org/std/primitive.char.html) — is_ascii_digit, is_ascii_alphabetic, etc.
- [Crafting Interpreters — Scanning](https://craftinginterpreters.com/scanning.html) — the scanning pattern in depth

