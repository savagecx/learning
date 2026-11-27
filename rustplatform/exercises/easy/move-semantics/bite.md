# Move Semantics

- Level: easy
- Author: Pybites

## Instructions
In Python, assignment creates a new reference to the same object:

```python
names = ["Alice", "Bob"]
guests = names  # both point to the same list
guests.append("Charlie")
print(names)  # ["Alice", "Bob", "Charlie"] — same list!
```

This works because Python uses reference counting and garbage collection. When no references remain, the GC cleans up.
Rust has no garbage collector. Instead, every value has exactly **one owner**. When you assign a value to another variable, ownership **moves**:

```rust
let names = vec!["Alice", "Bob"];
let guests = names;  // ownership moves to guests
// println!("{:?}", names);  // ERROR: names was moved
println!("{:?}", guests);    // OK: guests owns the data
```

After the move, `names` is no longer valid. This isn't a limitation — it's how Rust guarantees memory safety without a GC. When `guests` goes out of scope, the memory is freed. No double-free bugs, no use-after-free.

### Why moves matter

Moves prevent bugs that are common in other languages:

```rust
fn process(data: Vec) {
    // data is dropped at end of function
}

let numbers = vec![1, 2, 3];
process(numbers);
// numbers is gone — can't accidentally use freed memory
```

In C, you might accidentally use `numbers` after `process` freed it. In Python, the GC keeps it alive. Rust catches the bug at compile time.

### The move happens at assignment

The key insight: **assignment transfers ownership**. This applies to:
- `let b = a;`
- Passing to functions: `foo(a);`
- Returning from functions: `return a;`

---


## Your Task

Implement functions that demonstrate move semantics:

1. `take_ownership(items: Vec<String>) -> usize` — takes ownership of a vector and returns its length
2. `move_and_return(text: String) -> String` — takes ownership and returns the value wrapped in brackets: `"[text]"`
3. `swap_ownership(a: String, b: String) -> (String, String)` — takes two strings and returns them swapped: `(b, a)`


---


## Example


```rust
let items = vec!["one".to_string(), "two".to_string()];
assert_eq!(take_ownership(items), 2);
// items is no longer valid here

let text = String::from("hello");
let result = move_and_return(text);
assert_eq!(result, "[hello]");

let (x, y) = swap_ownership("first".to_string(), "second".to_string());
assert_eq!(x, "second");
assert_eq!(y, "first");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), the tokenizer produces a `Vec<Token>` that moves into the parser — ownership transfer ensures the tokens can't be accidentally modified after handoff.


---


## Further Reading


- [The Rust Book — What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — the ownership rules
- [The Rust Book — Move Semantics](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move) — how moves work

