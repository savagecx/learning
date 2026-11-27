# sort: Sort Lines

- Level: medium
- Author: Pybites

## Instructions
🎯 `sort` orders lines alphabetically; `sort -n` orders them as numbers. The difference matters: as text, `"10"` sorts *before* `"9"` (it compares character by character), with -n 9 does come before 10.
In Python you can accomplish this using the `key` argument of the `sorted` built-in:
def sort_lines(text: str, numeric: bool = False) -> list[str]:
    lines = text.splitlines()
    return sorted(lines, key=int) if numeric else sorted(lines)


### Sorting needs an owned, mutable collection


### 

To sort a `Vec` in-place you need to use `mut`:
let mut v = vec!["banana", "apple", "cherry"];
v.sort();   // ["apple", "banana", "cherry"] — lexical, like Python's sorted()


### `sort_by_key` — sort by a derived value

Lexical order puts `"10"` before `"9"`. For numeric order, you can use `sort_by_key` which takes a closure mapping each element to something comparable, similar how we use `key` in Python:
let mut v = vec!["ccc", "a", "bb"];
v.sort_by_key(|s| s.len());   // ["a", "bb", "ccc"] — neutral: order by length

Parsing a `&str` to a number returns a `Result`, so you'll need to turn that into a plain comparable key that treats non-numbers as `0` (the `parse` reference below points the way).

---


## Your Task

Implement `sort_lines(text: &str, numeric: bool) -> Vec<&str>`. When `numeric` is true, sort by integer value where non-numbers count as `0`.

---


## Example

assert_eq!(sort_lines("banana\napple\ncherry", false), vec!["apple", "banana", "cherry"]);
assert_eq!(sort_lines("10\n9\n100\n2", true), vec!["2", "9", "10", "100"]);


---


## Further Reading


- [slice::sort](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) — stable in-place sort
- [slice::sort_by_key](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key) — sort by a derived key
- [str::parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse) — parse a string into a number

