# Result Combinators

- Level: medium
- Author: Pybites

## Instructions
`Result<T, E>` is like `Option`, but carries error information. The same combinator patterns apply.

### Quick recap


```rust
enum Result {
    Ok(T),   // success with value
    Err(E),  // failure with error
}
```

In Python, you'd use exceptions. Rust uses return values — errors are data, not control flow.

### `.map()` — transform success value


```rust
let result: Result = Ok(5);
let doubled = result.map(|n| n * 2);  // Ok(10)

let result: Result = Err("oops".to_string());
let doubled = result.map(|n| n * 2);  // Err("oops") — unchanged
```


### `.map_err()` — transform the error


```rust
let result: Result = Err("not found");
let better = result.map_err(|e| format!("Error: {}", e));
// Err("Error: not found")
```

Useful for converting between error types.

### `.and_then()` — chain Result-returning functions


```rust
fn parse(s: &str;) -> Result {
    s.parse().map_err(|_| "parse error".to_string())
}

fn validate(n: i32) -> Result {
    if n > 0 { Ok(n) } else { Err("must be positive".to_string()) }
}

let result = parse("42").and_then(validate);  // Ok(42)
let result = parse("-5").and_then(validate);  // Err("must be positive")
let result = parse("abc").and_then(validate); // Err("parse error")
```


### `.unwrap_or()` and `.unwrap_or_else()`


```rust
let value = result.unwrap_or(0);  // 0 if Err
let value = result.unwrap_or_else(|e| {
    eprintln!("Error: {}", e);
    0
});
```


### `.ok()` — convert Result to Option


```rust
let result: Result = Ok(42);
let option: Option = result.ok();  // Some(42)

let result: Result = Err("nope");
let option: Option = result.ok();  // None — error is discarded
```


---


## Your Task


1. `safe_sqrt(n: f64) -> Result<f64, String>` — return sqrt if n >= 0, else Err("negative input")
2. `parse_positive(s: &str) -> Result<u32, String>` — parse and validate > 0
3. `chain_operations(s: &str) -> Result<f64, String>` — parse to i32, validate positive, then sqrt
4. `result_to_option(r: Result<i32, String>) -> Option<i32>` — convert, discarding error


---


## Example


```rust
assert_eq!(safe_sqrt(4.0), Ok(2.0));
assert_eq!(safe_sqrt(-1.0), Err("negative input".to_string()));

assert_eq!(parse_positive("42"), Ok(42));
assert!(parse_positive("0").is_err());
assert!(parse_positive("-5").is_err());
assert!(parse_positive("abc").is_err());

assert_eq!(chain_operations("16"), Ok(4.0));
assert!(chain_operations("-16").is_err());

assert_eq!(result_to_option(Ok(42)), Some(42));
assert_eq!(result_to_option(Err("error".to_string())), None);
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll compose Result operations in your parser pipeline — tokenize, then parse, then validate — each step returning `Result` and chaining cleanly with `?`.


---


## Further Reading


- [Result in std](https://doc.rust-lang.org/std/result/enum.Result.html) — all methods
- [Rust Book — Recoverable Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

