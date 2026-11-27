# uniq -c: Count Adjacent Duplicates

- Level: medium
- Author: Pybites

## Instructions
🎯 `uniq -c` collapses *consecutive* duplicate lines into one, prefixed with how many times it repeated.
The "consecutive" part is the catch: `uniq` only looks at neighbors, so `a a b a` becomes `2 a`, `1 b`, `1 a` (two separate runs of `a`). 
In Unix:
$ echo -e "tag1\ntag2\ntag1\ntag4\ntag2\ntag1" > tags.txt
$ sort tags.txt|uniq -c
   3 tag1
   2 tag2
   1 tag4

And in Python:
from itertools import groupby


def uniq_c(text: str) -> list[tuple[int, str]]:
    # need to sort the lines first to group them correctly
    sorted_lines = sorted(text.splitlines())
    # below takes the len of the values, intermediate results:
    # ('tag1', ['tag1', 'tag1', 'tag1'])
    # ('tag2', ['tag2', 'tag2'])
    # ('tag4', ['tag4'])
    return [(len(list(g)), key) for key, g in groupby(sorted_lines)]


assert uniq_c("tag1\ntag2\ntag1\ntag4\ntag2\ntag1") == [
    (3, "tag1"),
    (2, "tag2"),
    (1, "tag4"),
]


### `Vec::last_mut` — edit the final element in place

Walk the lines and look at what you last pushed. If the current line continues the run, bump its count; otherwise start a new run. `last_mut` on Vec can help you with this; it gives you a mutable reference to the final element (or `None` if the `Vec` is empty). For example:
let mut v = vec![1, 2, 3];
if let Some(last) = v.last_mut() {
    *last += 10;   // v is now [1, 2, 13]
}


### Match guards — a condition on a pattern

A **match guard** is an extra `if` attached to a pattern; so now you can have the arm match only when both the pattern *and* the guard hold:
match Some(4) {
    Some(x) if x % 2 == 0 => "even",
    _ => "odd or empty",
}

This can be useful to answer: "does the last element equal this line?"
Not required, but it can make your solution more concise here.

---


## Your Task

Implement `uniq_c(text: &str) -> Vec<(usize, &str)>`. Collapse runs of identical adjacent lines into `(count, line)` pairs, in order. Empty input → empty `Vec`.

---


## Example

assert_eq!(uniq_c("a\na\nb\nb\nb\na"), vec![(2, "a"), (3, "b"), (1, "a")]);
assert_eq!(uniq_c("x"), vec![(1, "x")]);


---


## Further Reading


- [slice::last_mut](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut) — mutable reference to the last element
- [Match guards](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#extra-conditionals-with-match-guards) — conditions on match arms

