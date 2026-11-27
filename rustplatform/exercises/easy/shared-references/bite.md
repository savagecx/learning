# Shared References

- Level: easy
- Author: Pybites

## Instructions
Moving ownership everywhere gets tedious. What if you just want to *look* at a value without taking it?
In Python, everything is passed by reference automatically:

```python
def print_length(items):
    print(len(items))  # just reads, doesn't modify

names = ["Alice", "Bob"]
print_length(names)
print(names)  # still valid
```

In Rust, you **borrow** with `&`:

```rust
fn print_length(items: &Vec;) {
    println!("{}", items.len());
}

let names = vec!["Alice".to_string(), "Bob".to_string()];
print_length(&names;);  // lend with &
println!("{:?}", names);  // still ours
```


### The borrowing rules

A shared reference (`&T`) is **read-only**. You can have as many as you want:

```rust
let text = String::from("hello");
let r1 = &text;
let r2 = &text;
let r3 = &text;
println!("{} {} {}", r1, r2, r3);  // all valid
```

But you cannot modify through a shared reference:

```rust
fn try_modify(s: &String;) {
    // s.push_str(" world");  // ERROR: cannot borrow as mutable
}
```


### Why this matters

Shared references guarantee that the data won't change while you're reading it. In Python, another thread could modify a list while you're iterating — leading to subtle bugs. Rust prevents this at compile time.

### Dereferencing

Sometimes you need to access the value behind a reference:

```rust
let x = 5;
let r = &x;
assert_eq!(*r, 5);  // *r dereferences to get the value
```

For method calls, Rust auto-dereferences, so `r.to_string()` works without `(*r).to_string()`.

---


## Your Task


1. `count_words(text: &String) -> usize` — count words in a borrowed string (split on whitespace)
2. `find_longest(words: &Vec<String>) -> &String` — return a reference to the longest string in the vector. Note: you'll likely reach for `.unwrap()` here — that's fine for this exercise, but in production code you'd return `Option<&String>` to handle the empty case safely. We'll cover that pattern in the error handling track.
3. `contains_value(items: &Vec<i32>, target: &i32) -> bool` — check if the vector contains the target value


---


## Example


```rust
let text = String::from("hello world from rust");
assert_eq!(count_words(&text;), 4);

let words = vec!["short".to_string(), "medium".to_string(), "longest".to_string()];
assert_eq!(find_longest(&words;), "longest");

let numbers = vec![1, 2, 3, 4, 5];
assert!(contains_value(&numbers;, &3));
assert!(!contains_value(&numbers;, &10));
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), `&self` methods like `peek()` and `is_at_end()` let you inspect parser state without consuming it — reading tokens without giving up access.


---


## Further Reading


- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — shared references
- [The Rust Book — The Rules of References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references) — borrowing rules

