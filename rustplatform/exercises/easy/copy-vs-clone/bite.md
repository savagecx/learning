# Copy vs Clone

- Level: easy
- Author: Pybites

## Instructions
In the previous exercise, you learned that assignment moves ownership. But wait — this works fine:

```rust
let x = 42;
let y = x;
println!("{}", x);  // still valid!
```

Why didn't `x` move? Because integers implement the `Copy` trait.

### Copy types: stack-only data

Types that are small and live entirely on the stack can be **copied** instead of moved:

```rust
let a: i32 = 5;
let b = a;  // a is copied, not moved
// both a and b are valid
```

Copy types include:
- All integer types (`i32`, `u64`, etc.)
- Floating point (`f32`, `f64`)
- Booleans (`bool`)
- Characters (`char`)
- Tuples of Copy types: `(i32, bool)`

### Non-Copy types: heap data

Types that manage heap memory cannot be `Copy` — they must be **moved** or explicitly **cloned**:

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED
// println!("{}", s1);  // ERROR

let s3 = String::from("hello");
let s4 = s3.clone();  // explicit deep copy
println!("{} {}", s3, s4);  // both valid
```


### Python comparison

In Python, everything is a reference. "Copying" is actually aliasing:

```python
x = [1, 2, 3]
y = x        # y points to same list
y.append(4)
print(x)     # [1, 2, 3, 4] — oops!

import copy
z = copy.copy(x)  # explicit shallow copy
```

Rust makes this explicit: small stack values copy automatically, heap values require `.clone()`.

### The Clone trait

`.clone()` creates a deep copy. It's explicit because it might be expensive:

```rust
let big_vec = vec![0; 1_000_000];
let copy = big_vec.clone();  // allocates another million integers
```


---


## Your Task


1. `double_number(n: i32) -> (i32, i32)` — takes a number, returns `(original, doubled)`. Since `i32` is `Copy`, you can use `n` multiple times.
2. `clone_and_modify(items: &Vec<i32>) -> Vec<i32>` — returns a new vector with 100 added to each element. Original stays unchanged.
3. `sum_of_copies(a: i32, b: i32, c: i32) -> i32` — returns twice the sum of all three arguments (uses each value twice — only works because i32 is Copy)


---


## Example


```rust
let (orig, doubled) = double_number(21);
assert_eq!(orig, 21);
assert_eq!(doubled, 42);

let numbers = vec![1, 2, 3];
let modified = clone_and_modify(&numbers;);
assert_eq!(numbers, vec![1, 2, 3]);  // unchanged
assert_eq!(modified, vec![101, 102, 103]);

assert_eq!(sum_of_copies(1, 2, 3), 12);
```


---


## Further Reading


- [The Rust Book — Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) — which types are Copy
- [The Rust Book — Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html) — explicit deep copying

