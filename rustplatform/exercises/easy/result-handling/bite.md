# Result Handling

- Level: easy
- Author: Pybites

## Instructions
🎯 Python uses exceptions for error handling:

```python
def withdraw(balance, amount):
    if amount > balance:
        raise ValueError("insufficient funds")
    return balance - amount

try:
    remaining = withdraw(50, 100)
except ValueError as e:
    print(e)
```

Rust takes a different approach: errors are **values**, not exceptions. Instead of raising and catching, you return a `Result`:

```rust
fn withdraw(balance: i32, amount: i32) -> Result {
    if amount > balance {
        Err("insufficient funds".to_string())
    } else {
        Ok(balance - amount)
    }
}
```

`Result<T, E>` is an enum with two variants:
- `Ok(T)` — success, carrying the value
- `Err(E)` — failure, carrying the error
The caller must handle both cases. The compiler won't let you use the value without checking whether the operation succeeded:

```rust
match withdraw(100, 30) {
    Ok(remaining) => println!("Balance: {}", remaining),
    Err(e) => println!("Error: {}", e),
}
```

This is fundamentally different from Python's exceptions: errors can't be silently ignored. If you don't handle the `Result`, the compiler warns you.

---


## Your Task

Implement `divide(a: i32, b: i32) -> Result<i32, String>`:

- Return `Ok(a / b)` if `b` is not zero
- Return `Err("division by zero".to_string())` if `b` is zero


---


## Example


```rust
assert_eq!(divide(10, 2), Ok(5));
assert_eq!(divide(7, 2), Ok(3));
assert_eq!(divide(10, 0), Err("division by zero".to_string()));
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll upgrade your tokenizer to return `Result<Vec<Token>, JsonError>` — so invalid JSON produces clear, structured errors instead of panics.


---


## Further Reading


- [The Rust Book — Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) — Result and error handling
- [The Rust Book — To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html) — when to use Result vs panic

