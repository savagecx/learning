# Fighting the Borrow Checker

- Level: medium
- Author: Pybites

## Instructions
In Python, you can freely mix reading and modifying data:

```python
items = [1, 2, 3]
first = items[0]     # read
items.append(4)      # modify while holding reference
print(first)         # works fine... usually

# But this can cause subtle bugs:
d = {"key": [1, 2]}
ref = d["key"]       # reference to the list
d["key"] = [99]      # replace the list
ref.append(3)        # ref still points to old list!
```

Python catches some issues at runtime, but many aliasing bugs silently corrupt data. Rust catches ALL these at compile time with the borrow checker.
Every Rust beginner fights the borrow checker. The errors look cryptic at first, but they always point to real bugs that would cause crashes in other languages.

### Error: "cannot borrow as mutable because it is also borrowed as immutable"


```rust
let mut v = vec![1, 2, 3];
let first = &v;[0];       // immutable borrow
v.push(4);               // ERROR: mutable borrow while immutable exists
println!("{}", first);   // first is still in use
```

**Why?** `push` might reallocate the vector, invalidating `first`. Rust catches this at compile time; C would crash at runtime.
**Fix:** finish using the immutable borrow before mutating:

```rust
let mut v = vec![1, 2, 3];
let first = v[0];        // copy the value (i32 is Copy)
v.push(4);               // now safe
println!("{}", first);
```


### Error: "cannot move out of index"


```rust
let names = vec!["Alice".to_string()];
let first = names[0];    // ERROR: cannot move out of Vec
```

**Why?** Taking ownership of `names[0]` would leave a "hole" in the vector.
**Fix:** borrow or clone:

```rust
let first = &names;[0];           // borrow
let first = names[0].clone();    // or clone to own
```


### Error: "value borrowed here after move"


```rust
let s = String::from("hello");
let s2 = s;              // s moved to s2
println!("{}", s);       // ERROR: s was moved
```

**Fix:** clone if you need both:

```rust
let s = String::from("hello");
let s2 = s.clone();
println!("{} {}", s, s2);
```


### The pattern: think about who needs what

Before writing code, ask:
- Do I need ownership or just a view?
- Am I done reading before I mutate?
- Can I copy/clone to avoid borrow conflicts?

---


## Your Task


1. `first_and_len(items: &Vec<i32>) -> (i32, usize)` — return the first element and length (notice how `i32` being `Copy` makes this straightforward)
2. `push_and_get_first(items: &mut Vec<i32>, value: i32) -> i32` — push a value, then return the first element
3. `clone_and_extend(original: &Vec<String>, suffix: &str) -> Vec<String>` — clone the vector and append suffix to each string


---


## Example


```rust
let nums = vec![10, 20, 30];
let (first, len) = first_and_len(&nums;);
assert_eq!(first, 10);
assert_eq!(len, 3);

let mut items = vec![1, 2, 3];
let first = push_and_get_first(&mut; items, 4);
assert_eq!(first, 1);
assert_eq!(items, vec![1, 2, 3, 4]);

let words = vec!["hello".to_string(), "world".to_string()];
let extended = clone_and_extend(&words;, "!");
assert_eq!(extended, vec!["hello!", "world!"]);
assert_eq!(words, vec!["hello", "world"]);  // original unchanged
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), the borrow checker keeps your parser honest — you can't peek at the current token while also advancing past it, preventing subtle bugs that would slip through in Python.


---


## Further Reading


- [The Rust Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — borrow rules
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md) — deeper understanding

