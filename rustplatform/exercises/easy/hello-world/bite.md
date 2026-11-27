# Hello, World!

- Level: easy
- Author: Tanner-Ray-Martin

## Instructions
🎯 In Python, `"Hello, World!"` is just a `str` — there's only one string type and you rarely think about it.
Rust has **two** main string types, and understanding the difference early will save you a lot of confusion:

- **`&str`** (string slice) — a *borrowed* reference to string data. Think of it like a read-only view. String literals like `"hello"` are `&str` by default.
- **`String`** (owned string) — a *heap-allocated*, growable string that you own and can modify. Closer to Python's `str` in terms of what you can do with it.

The function signature here asks you to return a `String`, not a `&str`. That means you need to convert the string literal into an owned `String`.
**Common ways to do this:**

```rust
"hello".to_string()    // calls the ToString trait
String::from("hello")  // explicit construction
"hello".into()         // uses the Into trait (type must be inferrable)
```

All three are equivalent — pick whichever reads best to you. You'll see `.to_string()` and `String::from()` most often in Rust codebases.

---


## Your Task

Implement the `hello_world` function so it returns `"Hello, World!"` as an owned `String`.

---


## Example


```rust
assert_eq!(hello_world(), "Hello, World!");
```


---


## Further Reading


- [The Rust Book — What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — foundational for understanding `String` vs `&str`
- [The Rust Book — The String Type](https://doc.rust-lang.org/book/ch08-02-strings.html) — deep dive into how Rust strings work under the hood

