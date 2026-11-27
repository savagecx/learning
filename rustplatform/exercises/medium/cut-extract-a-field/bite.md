# cut: Extract a Field

- Level: medium
- Author: Pybites

## Instructions
🎯 `cut -d: -f1 /etc/passwd` pulls the first colon-separated field (the username) from each line. In Python you'd split and index:
def cut(text: str, delim: str, field: int):  # field is 1-based
    if field == 0:
        raise ValueError("field values may not include zero")
    out = []
    for line in text.splitlines():
        parts = line.split(delim)
        if len(parts) >= field:
            out.append(parts[field - 1])
    return out


assert cut("foo:bar\nbaz:qux", ":", 1) == ["foo", "baz"]
assert cut("foo:bar\nbaz:qux", ":", 2) == ["bar", "qux"]
assert cut("foo:bar\nbaz:qux", ":", 3) == []
assert cut("foo:bar\nbaz:qux", ":", 4) == []
try:
    assert cut("foo:bar\nbaz:qux", ":", 0)
except ValueError as e:
    assert str(e) == "field values may not include zero"
else:
    raise AssertionError("Expected ValueError")

Two cases pull in different directions: a *missing field* (some lines are too short) is normal, those we skip.
However `field == 0` is a *bad request*. Real `cut` rejects it: `cut: [-bcf] list: values may not include zero`. Rust lets you model that split cleanly: one path returns data, the other returns an error.

### `.nth()` returns an `Option`

Splitting a string gives an iterator over the parts, and `.nth(i)` pulls the `i`-th (0-based), returning an `Option`: `Some(part)` if it exists, `None` if there were too few parts. For example:
let mid = "a-b-c".split('-').nth(1); // Some("b")
let off = "a-b-c".split('-').nth(9); // None — out of range, not a crash

That `Option` is Rust refusing to let you ignore the out-of-range case, where Python could surprise you with an `IndexError` at runtime.

### `filter_map` drops the misses

`filter_map` keeps every `Some(...)` and silently drops the `None`s in one pass, exactly the "skip lines without that field" behavior. For example:
let evens: Vec<i32> = ["1", "x", "4"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect(); // [1, 4] — "x" failed to parse and was dropped


### A typed error, and `ok_or` to reach it

Model the one invalid input with your own error type. An enum is great for this; it makes the failure modes explicit and testable:
#[derive(Debug, PartialEq)]
enum CutError {
    ZeroField,
}

The function now returns `Result<Vec<&str>, CutError>`. To convert the 0-guard into that error, turn an `Option` into a `Result` with [`Option::ok_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or), then `?` to bail early:
let n = "abc".find('z').ok_or(CutError::ZeroField)?; // find returns Option

To get that `Option` in the first place: `field` is a `usize`, and subtracting `1` from `0` would underflow rather than go negative.
Worth checking out in this context is [usize::checked_sub](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_sub) - docs: *Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.*

---


## Your Task

Implement `cut(text: &str, delim: char, field: usize) -> Result<Vec<&str>, CutError>`. `field` is 1-based.
Return `Err(CutError::ZeroField)` when `field == 0`; otherwise return that field from each line as `Ok(...)`, skipping lines that don't have it.

---


## Example

assert_eq!(cut("root:x:0\ndaemon:x:1", ':', 1), Ok(vec!["root", "daemon"]));
assert_eq!(cut("root:x:0\ndaemon:x:1", ':', 3), Ok(vec!["0", "1"]));
assert_eq!(cut("root:x:0\ndaemon:x:1", ':', 5), Ok(Vec::<&str>::new()));
assert_eq!(cut("a:b", ':', 0), Err(CutError::ZeroField));


---


## Further Reading


- [Iterator::nth](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth) — the nth item as an Option
- [Iterator::filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) — map and drop None in one step
- [Option::ok_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) — turn `None` into your own error
- [usize::checked_sub](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_sub) — subtraction that returns `None` on underflow
- [The ? operator](https://doc.rust-lang.org/std/result/index.html) — short-circuiting on Err

