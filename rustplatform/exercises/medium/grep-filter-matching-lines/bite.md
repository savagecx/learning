# grep: Filter Matching Lines

- Level: medium
- Author: Pybites

## Instructions
🎯 `grep` keeps the lines that match a pattern. Real `grep` also has flags: `-i` (ignore case) and `-v` (invert = keep the *non*-matches). 
In Python:
def grep(
    text: str, pattern: str, ignore_case: bool = False, invert: bool = False
) -> Generator[str, None, None]:
    lines = text.splitlines()
    if ignore_case:
        pattern = pattern.lower()
    for line in lines:
        hay = line.lower() if ignore_case else line
        found = pattern in hay
        keep = not found if invert else found
        if keep:
            yield line
        # or xor logic
        # if (pattern in hay) != invert:
        #    yield line



### Substring test and case folding

`line.contains(pattern)` is Rust's `pattern in line`. For case-insensitive matching, you can use `.to_lowercase()`.

### A note on the signature - what is 'a ? 

fn grep<'a>(text: &'a str, pattern: &str, ignore_case: bool, invert: bool) -> Vec<&'a str>

In Python, the returned list owns its strings, and the garbage collector (GC) keeps `text` alive as long as anything still points at it. You don't have to worry about it.
Rust has no GC, so it has to *prove* at compile time that no reference outlives the data it points into.
A `&str` isn't a copy of the text, it's a borrow, a (pointer, length) view into bytes someone else owns. The lines this function returns are slices *of `text`*: no allocation, just windows into the original string. So those returned slices are only valid as long as `text` is. That relationship is exactly what a *lifetime* names.
Why must you write it out here? When a function takes a single reference, Rust silently assumes the output borrows from it (lifetime *elision*). But `grep` takes **two** `&str` inputs (`text` and `pattern`), and now the compiler can't guess which one the result points into.
So you annotate with `'a` on `text` and on the return indicating that the output borrows from `text`, as opposed to `pattern` (not `'a`) which is read but never returned. Taking the lifetime out and you'll get a compiler error: `missing lifetime specifier`.
The alternative is to return `Vec<String>` (owned, GC-like) by copying each matching line. Then you don't need a lifetime, but it would make an allocation per line which is expensive when matching many lines. 
Borrowing from the input is the idiomatic choice for a grep, so keep the annotation as is for now. The *Lifetimes* track will dig deeper into this so you'll fully understand and practice it. It'll release after this Unix tools one.

---


## Your Task

Implement `grep` with the signature above. Return the lines that match `pattern`, honoring `ignore_case` and `invert` as we've seen in the Python version. Order is preserved.

---


## Example

assert_eq!(grep("foo\nbar\nFOObar", "foo", false, false), vec!["foo"]);
assert_eq!(grep("foo\nbar\nFOObar", "foo", true, false), vec!["foo", "FOObar"]);
assert_eq!(grep("foo\nbar\nFOObar", "foo", false, true), vec!["bar", "FOObar"]);


---


## Further Reading


- [str::contains](https://doc.rust-lang.org/std/primitive.str.html#method.contains) — substring search
- [Iterator::filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) — keep items matching a predicate
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) — the Rust Book on `'a` and elision

