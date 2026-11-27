# tr: Translate and Delete Characters

- Level: easy
- Author: Pybites

## Instructions
🎯 `tr` rewrites text one character at a time: `tr 'a-z' 'A-Z'` upper-cases, `tr -d` drops characters.
In Python you'd reach for `str.replace`, for example:
def translate(text: str, frm: str, to: str) -> str:
    return text.replace(frm, to)

def delete(text: str, ch: str) -> str:
    return text.replace(ch, "")

Rust has no "string-replace-with-a-char" shortcut here, you'll need to walk the characters yourself, because ...

### `char` is not a one-character string

In Rust `'l'` is a `char` - a single Unicode scalar value - while `"l"` is a `&str`. Note the difference in single vs double quotes used to denote each.
They're different types. `.chars()` turns a `&str` into an iterator of `char`:
for c in "hi".chars() { /* c: char */ }


### Transform each char, then rebuild the string

`.map()` applies a function to every item. To rewrite characters, map each one to its replacement (or leave it unchanged):
let masked: String = "a1b2".chars().map(|c| if c.is_numeric() { '#' } else { c }).collect();
// "a#b#"


### Keep only some chars

`.filter()` drops items that fail a test. The `|&c|` pattern copies the `char` out of the reference the closure receives:
let digits: String = "1-2-3".chars().filter(|&c| c != '-').collect();   // "123"


### `.collect::<String>()`

An iterator of `char` collects straight into a `String`. The target type is inferred from the function's return type here, so a plain `.collect()` should be enough here.

---


## Your Task

Implement both:

- `translate(text: &str, from: char, to: char) -> String` — every `from` becomes `to`.
- `delete(text: &str, ch: char) -> String` — every `ch` is dropped.

A char that doesn't appear leaves the text unchanged. Empty input → empty `String`.

---


## Example

assert_eq!(translate("hello", 'l', 'L'), "heLLo");
assert_eq!(delete("h.e.l.l.o", '.'), "hello");


---


## Further Reading


- [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars) — iterate over characters
- [Iterator::map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) — transform each item
- [Iterator::filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) — keep items that pass a test

