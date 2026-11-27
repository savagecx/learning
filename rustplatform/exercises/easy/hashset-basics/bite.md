# HashSet Basics

- Level: easy
- Author: Pybites

## Instructions
In Python, sets provide unique collections with fast membership testing:

```python
tags = {"rust", "python", "coding"}
tags.add("rust")  # no duplicate
print("rust" in tags)  # O(1) lookup

a = {1, 2, 3}
b = {2, 3, 4}
print(a & b)  # intersection: {2, 3}
print(a | b)  # union: {1, 2, 3, 4}
print(a - b)  # difference: {1}
```

Rust's `HashSet<T>` works the same way.

### Creating and using HashSet


```rust
use std::collections::HashSet;

let mut tags: HashSet = HashSet::new();
tags.insert("rust".to_string());
tags.insert("rust".to_string());  // ignored - already exists
assert_eq!(tags.len(), 1);

if tags.contains("rust") {
    println!("Found it!");
}
```


### Set operations


```rust
let a: HashSet = [1, 2, 3].into_iter().collect();
let b: HashSet = [2, 3, 4].into_iter().collect();

let intersection: HashSet<_> = a.intersection(&b;).copied().collect();
let union: HashSet<_> = a.union(&b;).copied().collect();
let difference: HashSet<_> = a.difference(&b;).copied().collect();
```


### From iterator


```rust
let numbers = vec![1, 2, 2, 3, 3, 3];
let unique: HashSet = numbers.into_iter().collect();
assert_eq!(unique.len(), 3);
```


---


## Your Task


1. `unique_items(items: Vec<i32>) -> Vec<i32>` — return unique items (preserve order of first occurrence)
2. `common_elements(a: &[i32], b: &[i32]) -> Vec<i32>` — find elements in both (sorted output)
3. `is_subset(subset: &[i32], superset: &[i32]) -> bool` — check if all elements of subset are in superset


---


## Example


```rust
assert_eq!(unique_items(vec![3, 1, 2, 1, 3]), vec![3, 1, 2]);

let common = common_elements(&[1, 2, 3, 4], &[3, 4, 5, 6]);
assert_eq!(common, vec![3, 4]);

assert!(is_subset(&[1, 2], &[1, 2, 3, 4]));
assert!(!is_subset(&[1, 5], &[1, 2, 3, 4]));
```


---


## Further Reading


- [The Rust Book — HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [HashSet operations](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection)

