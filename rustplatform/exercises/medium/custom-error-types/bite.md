# Custom Error Types

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, you define custom exceptions by subclassing:

```python
class CalcError(Exception):
    pass

class DivisionByZero(CalcError):
    pass

class ParseError(CalcError):
    def __init__(self, token):
        self.token = token
```

Different error types, but they're all classes in an inheritance hierarchy. You catch the base class or specific ones.
Rust uses **enums** for error types instead of class hierarchies:

```rust
#[derive(Debug, PartialEq, Eq)]
enum CalcError {
    DivisionByZero,
    ParseInt(String),
    BadFormat,
}
```

Each variant is a distinct error case. The enum carries data when needed (`ParseInt(String)` stores the bad token). No inheritance, no base class — the `match` expression handles each case exhaustively.

### The ? operator

In exercise 09, you returned `Result` manually. Rust's `?` operator chains operations that might fail:

```rust
fn process(input: &str;) -> Result {
    let parsed = parse(input)?;   // if Err, return it immediately
    let checked = validate(parsed)?;   // if Err, return it immediately
    transform(checked)
}
```

`?` is like Python's implicit exception propagation, but explicit — you see exactly where errors can occur and how they flow.

### map_err for error conversion

When the error types don't match directly, `map_err` converts:

```rust
user_input
    .parse::()
    .map_err(|_| AppError::InvalidInput(user_input.to_string()))
```

This turns a `std::num::ParseFloatError` into your `AppError::InvalidInput`. Python's equivalent would be catching one exception and raising another.

---


## Your Task


1. Define the `CalcError` enum (already provided) with variants: `DivisionByZero`, `ParseInt(String)`, `BadFormat`
2. Implement three functions:
3. `parse_int(tok: &str) -> Result<i32, CalcError>` — trim and parse; return `ParseInt(token)` on failure
4. `safe_div(a: i32, b: i32) -> Result<i32, CalcError>` — return `DivisionByZero` if `b == 0`
5. `eval_div(expr: &str) -> Result<i32, CalcError>` — parse `"a / b"` format, split on `/`, parse both sides, divide. Return `BadFormat` if not exactly two parts.


---


## Example


```rust
use CalcError::*;
assert_eq!(eval_div("12/3"), Ok(4));
assert_eq!(eval_div(" 10 / 2 "), Ok(5));
assert_eq!(eval_div("7/0"), Err(DivisionByZero));
assert_eq!(eval_div("x/3"), Err(ParseInt("x".into())));
assert_eq!(eval_div("1/2/3"), Err(BadFormat));
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll build a `JsonError` enum with variants like `UnexpectedToken`, `UnterminatedString`, and `InvalidNumber` — giving users precise diagnostics when parsing fails.


---


## Further Reading


- [The Rust Book — The ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) — error propagation
- [The Rust Book — Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) — enums as error types
- [Result::map_err](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) — converting error types

