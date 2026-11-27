# Ownership and Borrowing

- Level: medium
- Author: Pybites

## Instructions
🎯 In Python, you never think about who "owns" a value:

```python
def label(item):
    return f"Product: {item}"

item = "Laptop"
tag = label(item)
print(item)  # still works — item wasn't consumed
```

Every variable is a reference to a garbage-collected object. Pass it to a function, return it, store it — the GC handles cleanup.
Rust has no garbage collector. Instead, it uses **ownership** — every value has exactly one owner, and when the owner goes out of scope, the value is dropped (freed). When you pass a value to a function, you **transfer ownership** (a "move"):

```rust
fn announce(item: String) -> String {
    format!("Now selling: {}!", item)
}

let item = String::from("Laptop");
let msg = announce(item);
// item is gone — it was moved into announce()
```


### Borrowing: using without owning

To use a value without taking ownership, you **borrow** it with `&`:

```rust
fn length(s: &String;) -> usize {
    s.len()
}

let name = String::from("Alice");
let len = length(&name;);    // borrow with &
println!("{}", name);       // still works — name wasn't moved
```

`&String` is a shared reference — read-only access. The original owner keeps the value.

### Mutable borrowing

To modify a borrowed value, use `&mut`:

```rust
fn append_exclaim(s: &mut; String) {
    s.push('!');
}

let mut msg = String::from("hello");
append_exclaim(&mut; msg);
assert_eq!(msg, "hello!");
```

Rust's rules: you can have **many shared borrows** (`&T`) or **one mutable borrow** (`&mut T`), but not both at the same time. This prevents data races at compile time.

---


## Your Task

Implement three functions demonstrating ownership, shared borrowing, and mutable borrowing:

1. `transfer_ownership(name: String) -> String` — takes ownership of a `String`, returns `"Welcome, {name}!"`
2. `longest_word_length(word1: &String, word2: &String) -> usize` — borrows two strings, returns the length of the longer one
3. `to_uppercase(text: &mut String)` — mutably borrows a string and converts it to uppercase in place


---


## Example


```rust
assert_eq!(
    transfer_ownership(String::from("Alice")),
    "Welcome, Alice!"
);

let word1 = String::from("borrow");
let word2 = String::from("rust");
assert_eq!(longest_word_length(&word1;, &word2;), 6);

let mut s = String::from("rust");
to_uppercase(&mut; s);
assert_eq!(s, "RUST");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), ownership rules govern how JSON strings flow from the tokenizer into the parser — you'll see exactly why Rust prevents use-after-move bugs.


---


## Further Reading


- [The Rust Book — What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — ownership rules and moves
- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — shared and mutable borrows

