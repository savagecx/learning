# The ? Operator

- Level: medium
- Author: Pybites

## Instructions
Chaining `.and_then()` works, but gets verbose. Rust's **`?` operator** is syntactic sugar for error propagation.

### Before: manual matching


```rust
fn read_config() -> Result {
    let file = match File::open("config.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let contents = match read_to_string(file) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    parse_config(&contents;)
}
```


### After: with `?`


```rust
fn read_config() -> Result {
    let file = File::open("config.txt")?;
    let contents = read_to_string(file)?;
    parse_config(&contents;)
}
```

The `?` does exactly what the `match` did: if `Ok`, unwrap the value; if `Err`, return early with the error.

### Python comparison

In Python, exceptions propagate automatically:

```python
def read_config():
    file = open("config.txt")  # raises if fails
    contents = file.read()      # raises if fails
    return parse_config(contents)
```

Rust's `?` gives you the same concise code, but the error handling is explicit in the return type.

### `?` works with Option too


```rust
fn get_first_char(s: Option) -> Option {
    let text = s?;  // return None if s is None
    text.chars().next()
}
```


### The return type must match

`?` only works when the function returns `Result` or `Option`. This won't compile:

```rust
fn main() {
    let f = File::open("test.txt")?;  // ERROR: main returns ()
}
```

Fix: use `.unwrap()`, `.expect()`, or change the return type.

### Error conversion with `From`

If your function returns `Result<T, MyError>` but you call something that returns `Result<T, OtherError>`, Rust can convert automatically if `MyError: From<OtherError>`:

```rust
impl From for MyError {
    fn from(e: io::Error) -> Self {
        MyError::Io(e)
    }
}
```


---


## Your Task


1. `parse_two_numbers(a: &str, b: &str) -> Result<(i32, i32), String>` — parse both, propagate errors with `?`
2. `sum_first_two(numbers: &[i32]) -> Option<i32>` — get first two elements and sum them (use `?` with Option)
3. `divide_parsed(a: &str, b: &str) -> Result<i32, String>` — parse both, then divide (handle division by zero)


---


## Example


```rust
assert_eq!(parse_two_numbers("10", "20"), Ok((10, 20)));
assert!(parse_two_numbers("10", "abc").is_err());

assert_eq!(sum_first_two(&[1, 2, 3]), Some(3));
assert_eq!(sum_first_two(&[1]), None);
assert_eq!(sum_first_two(&[]), None);

assert_eq!(divide_parsed("10", "2"), Ok(5));
assert!(divide_parsed("10", "0").is_err());
assert!(divide_parsed("abc", "2").is_err());
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), the `?` operator threads errors from tokenizer through parser to the caller — one clean pipeline, no manual error checking at every step.


---


## Further Reading


- [Rust Book — The ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- [Rust Book — To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)

