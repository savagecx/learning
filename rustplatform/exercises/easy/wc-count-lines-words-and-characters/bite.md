# wc: Count Lines, Words, and Characters

- Level: easy
- Author: Pybites

## Instructions
🎯 The Unix `wc` ("word count") tool reports how many lines, words, and characters a file holds.
In Python you'd lean on the string methods:
def wc(text):
    return (
        len(text.splitlines()),   # lines
        len(text.split()),        # words
        len(text),                # characters
    )

This is the first tool in the `unix-tools` track.
Every tool here follows the same shape: a **pure function** does the counting/transforming, and in a real CLI a thin wrapper would feed it text from a file or stdin.
*You implement and test just the pure function here; keeping the logic pure is what makes it testable, and exactly how you'd structure a real Rust CLI.*

### 


### Iterators + `.count()` replace `len(...)`

Python builds a throwaway list just to measure it (`len(text.split())`). Rust counts lazily: string-splitting methods hand back an *iterator*, and `.count()` walks it to a total without ever allocating a `Vec`:
let n = "a-b-c-d".split('-').count();   // 4

You need three iterators here: one that yields lines, one that yields whitespace-separated words, and one that yields characters. They all live on `str`; the docs linked below will point you to the right method names. Call `.count()` on each.
One subtlety for the word count: the whitespace splitter you want skips empty chunks, so `"  a   b  "` is two words, not a pile of empty strings, the same behavior as Python's argument-less `str.split()`.

### Returning a tuple

Rust tuples work like Python's: group a fixed number of values of possibly different types.
Here all three are `usize` (Rust's type for sizes and counts), so the return type is `(usize, usize, usize)`:
fn wc(text: &str) -> (usize, usize, usize) {
    // ...
    (lines, words, chars)
}


---


## Your Task

Implement `wc(text: &str) -> (usize, usize, usize)` returning `(lines, words, chars)`:

- **lines**: number of lines (use `.lines()`)
- **words**: number of whitespace-separated words
- **chars**: number of characters

An empty string has zero of everything.

---


## Example

assert_eq!(wc("hello world"), (1, 2, 11));
assert_eq!(wc("one two three\nfour five\n"), (2, 5, 24));
assert_eq!(wc(""), (0, 0, 0));


---


## Further Reading


- [str::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines) — iterate over lines
- [str::split_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) — split on any whitespace
- [Iterator::count](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) — consume an iterator and count its items

