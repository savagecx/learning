# sed: Find and Replace, Line by Line

- Level: medium
- Author: Pybites

## Instructions
🎯 `sed 's/old/new/'` is the find-and-replace you reach for daily (also works in Vim: `:s/old/new/`).
Two things define it: it works **one line at a time**, and by default it swaps only the **first** match on each line. Add the `g` (global) flag - `s/old/new/g` - to swap *every* match.
In Python you'd lean on `str.replace`, whose third argument is the replacement count:
def substitute(text: str, frm: str, to: str, global_: bool) -> str:
    count = -1 if global_ else 1            # -1 = all, 1 = just the first
    return "\n".join(line.replace(frm, to, count) for line in text.splitlines())

Rust gives you the same two behaviours as two different methods, and you choose between them.

### `str` has a real replace — unlike `tr`

In the `tr` bite you will write text a `char` at a time by hand. Here the unit is a *substring*, and `&str` hands you the method outright:
let shout = "na na na".replace("na", "YO");   // "YO YO YO" — every match

`replace` swaps **all** occurrences. Its sibling `replacen` takes a count and stops early:
let once = "na na na".replacen("na", "YO", 1); // "YO na na" — just the first

So the `global` flag is just a choice between these two, no manual counting needed.

### sed is line-oriented, so split, transform, rejoin

The catch: "first match" means first match *per line*, not first in the whole text.
So you can't run one `replace` over everything; you process each line on its own.

- `.lines()` gives you that iterator (you met it in `cat -n` and `sort`),
- `.map()` transforms each line,
- and `.join()` stitches the results back into one `String`:

let parts = ["one", "two", "three"];
let line = parts.join(", ");                   // "one, two, three"

`.lines()` drops the newlines on the way in, so rejoining with `"\n"` rebuilds the text.

### Mapping with a branch inside

Each line gets the *same* decision — all matches or just the first — driven by the `global` flag. An `if`/`else` is an expression in Rust, so it can be the whole body of the closure, returning one `String` either way:
let label = |n: i32| if n % 2 == 0 { "even" } else { "odd" };   // both arms: &str


---


## Your Task

Implement one function:

- `substitute(text: &str, from: &str, to: &str, global: bool) -> String`

For each line, replace `from` with `to`. When `global` is `true`, replace every match on the line; when `false`, replace only the first. Assume `from` is non-empty. Empty input → empty `String`.

---


## Example

assert_eq!(substitute("foo foo foo", "foo", "bar", false), "bar foo foo");
assert_eq!(substitute("foo foo foo", "foo", "bar", true), "bar bar bar");
assert_eq!(substitute("a a\nb a", "a", "X", false), "X a\nb X");   // first per line


---


## Further Reading


- [str::replace](https://doc.rust-lang.org/std/primitive.str.html#method.replace) — swap every occurrence of a pattern
- [str::replacen](https://doc.rust-lang.org/std/primitive.str.html#method.replacen) — swap at most the first *n*
- [str::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines) — iterate over lines, newlines stripped
- [slice::join](https://doc.rust-lang.org/std/primitive.slice.html#method.join) — glue pieces back together with a separator

