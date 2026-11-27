# Mutable References

- Level: medium
- Author: Pybites

## Instructions
Shared references (`&T`) are read-only. To modify borrowed data, you need a **mutable reference** (`&mut T`):

```rust
fn add_greeting(text: &mut; String) {
    text.push_str(", hello!");
}

let mut message = String::from("World");
add_greeting(&mut; message);
assert_eq!(message, "World, hello!");
```


### Python comparison

In Python, you can modify objects passed to functions because everything is a reference:

```python
def add_item(items, item):
    items.append(item)

my_list = [1, 2]
add_item(my_list, 3)
print(my_list)  # [1, 2, 3]
```

This is convenient but dangerous — any function might modify your data unexpectedly. Rust makes mutation explicit with `&mut`.

### The exclusivity rule

Rust enforces a critical safety rule: **you can have many shared references OR one mutable reference, but not both at the same time**.

```rust
let mut s = String::from("hello");

let r1 = &s;      // shared borrow
let r2 = &s;      // another shared borrow — OK
// let r3 = &mut; s;  // ERROR: can't borrow mutably while shared borrows exist

println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut; s;  // OK now — no active shared borrows
r3.push_str(" world");
```


### Why this matters

This rule prevents data races at compile time. In Python:

```python
items = [1, 2, 3]
for item in items:
    if item == 2:
        items.remove(item)  # modifying while iterating — bug!
```

Rust won't let you hold a shared reference (the iterator) while also holding a mutable reference (to remove items).

### The mut keyword

Note that both the variable AND the borrow must be mutable:

```rust
let mut x = 5;      // variable must be mut
let r = &mut; x;     // borrow must be &mut;
*r += 1;            // dereference to modify
```


---


## Your Task


1. `double_all(numbers: &mut Vec<i32>)` — double every number in the vector in place
2. `remove_negatives(numbers: &mut Vec<i32>)` — remove all negative numbers from the vector
3. `append_reversed(dest: &mut String, source: &str)` — append the reversed source string to dest


---


## Example


```rust
let mut nums = vec![1, 2, 3];
double_all(&mut; nums);
assert_eq!(nums, vec![2, 4, 6]);

let mut nums = vec![1, -2, 3, -4, 5];
remove_negatives(&mut; nums);
assert_eq!(nums, vec![1, 3, 5]);

let mut text = String::from("hello ");
append_reversed(&mut; text, "dlrow");
assert_eq!(text, "hello world");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), `&mut self` methods like `advance()` and `consume()` move the parser forward through the token stream — mutating state safely one step at a time.


---


## Further Reading


- [The Rust Book — Mutable References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references) — `&mut T`
- [The Rust Book — Data Races](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references) — why the exclusivity rule exists

