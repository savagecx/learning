# cat -n: Number the Lines

- Level: easy
- Author: Pybites

## Instructions
🎯 `cat -n` prints each line prefixed with its line number. In Python, `enumerate` with a start of 1 makes this easy and concise:
def cat_n(text):
    lines = text.splitlines()
    return "\n".join(f"{i}\t{line}" for i, line in enumerate(lines, start=1))

Rust has the same `enumerate` and like Python's `enumerate`, it pairs each item with its position, but you cannot override the starting int:
for (i, c) in ['a', 'b', 'c'].iter().enumerate() {
    println!("{}: {}", i + 1, c);
    // 1: a
    // 2: b
    // 3: c
}


### `format!` and `join` for text building

`format!` is Rust's f-string: it returns a new `String` from a template. `\t` is a tab, `{}` a placeholder. To assemble the final output, `map` each line to its numbered form, collect into a `Vec<String>`, and `join("\n")` them together, again calling it as a *method* rather in Rust.

---


## Your Task

Implement `cat_n(text: &str) -> String`. Number each line starting at `1`, formatted as `number`, a tab, then the line. Join lines with `\n` (no trailing newline). Note that for an empty input you need to return an empty string.

---


## Example

assert_eq!(cat_n("foo\nbar\nbaz"), "1\tfoo\n2\tbar\n3\tbaz");
assert_eq!(cat_n("only"), "1\tonly");


---


## Further Reading


- [Iterator::enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) — pair items with their index
- [format! macro](https://doc.rust-lang.org/std/macro.format.html) — build a String from a template
- [slice::join](https://doc.rust-lang.org/std/primitive.slice.html#method.join) — join with a separator

