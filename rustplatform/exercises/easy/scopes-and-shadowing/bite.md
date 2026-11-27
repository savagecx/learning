# Scopes and Shadowing

- Level: easy
- Author: dpdresser

## Instructions
🎯 In Python, variables "leak" out of blocks — there's no block scoping:

```python
if True:
    x = 42
print(x)  # 42 — x is still accessible
```

Rust is different. Curly braces `{}` create real **scopes**, and variables declared inside a scope are dropped when it ends:

```rust
{
    let x = 42;
    println!("{x}");  // fine
}
// println!("{x}");  // ERROR: x doesn't exist here
```

This isn't just a visibility rule — when a scope ends, Rust **frees the memory** for any owned values declared in it. This is how Rust manages memory without a garbage collector.

### Shadowing vs mutation

These look similar but are fundamentally different:

```rust
// Mutation: changes the value in the same variable
let mut x = 5;
x = 10;  // same variable, new value

// Shadowing: creates a NEW variable that happens to have the same name
let x = 5;
let x = 10;  // completely new variable, old one is gone
```

The key difference: **shadowing can change the type**. Mutation cannot:

```rust
let x = "42";          // x is &str;
let x = x.parse::().unwrap();  // x is now i32 — new variable, new type

let mut y = "42";
// y = 42;  // ERROR: can't change type of y
```

This is handy for transforming data step-by-step while reusing a meaningful name.

### Scopes and references interact

Here's where it gets subtle. A mutable reference to an outer variable still points to that outer variable, even when a new variable with the same name shadows it in an inner scope:

```rust
let mut x = 5;
let y = &mut; x;  // y points to the outer x

{
    let x = 10;  // shadows outer x, but y still points to outer x
    // y and the inner x are completely unrelated
}
```

Understanding this interaction between scopes, shadowing, and references is essential for reading (and debugging) Rust code.

---


## Your Task

Fix and complete the `scope_and_shadowing` function. The template has blanks (`_`) and errors for you to fix:

- Add the right `mut` keywords where needed
- Fill in the blanks to make the test pass
- The function returns a tuple of 4 values — read the test to understand what's expected

Trace through the code carefully: which `x` does `y` point to? What happens when `x` is shadowed in the inner scope?

---


## Example


```rust
assert_eq!(
    scope_and_shadowing(),
    (25, 10, String::from("outer-modified"), String::from("inner"))
);
```


---


## Further Reading


- [The Rust Book — Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — covers shadowing in the "Shadowing" section
- [The Rust Book — Understanding Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — how scopes determine when memory is freed

