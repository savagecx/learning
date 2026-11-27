# head & tail: First and Last N Lines

- Level: easy
- Author: Pybites

## Instructions
🎯 `head` prints the first few lines of its input; `tail` prints the last few. They mirror each other, so we'll build both.
In Python you'd slice from each end:
def head(text: str, n: int) -> list[str]:
    return text.splitlines()[:n]

def tail(text: str, n: int) -> list[str]:
    return text.splitlines()[-n:]

That slice materializes the whole list first. Both languages can avoid it. Python with `itertools.islice(lines, n)`, Rust with `.take(n)`.
That works for `head` because it stops early. With `tail` however you don't know where the end is until you've seen every line. That asymmetry (not the language) is what makes the two halves of this exercise different.

### 


### `head`: `.take(n)` stops early

Iterator adaptors are lazy: nothing runs until something consumes them. `.take(n)` yields at most `n` items, then stops pulling from the source, so you never touch the lines you don't need. Then use `.collect()` to materialize the items into a `Vec`: `let first_two: Vec<i32> = (1..1_000_000).take(2).collect();   // [1, 2]`

### `tail`: collect first, then slice from the end

You can't stop early going backwards; you don't know where the end is until you've seen every line. You can gather them into a `Vec` and index the tail.
Note that `lines.len() - n` would **panic** when `n` exceeds the line count. Look into `saturating_sub` (link below) for an idiomatic guard against unsigned underflow; it clamps at `0` instead of wrapping.
Note that in Python `lines[-0:]` is `lines[0:]` and it returns the *whole* list (for a 3 int list: `lines[-0:] # [1, 2, 3]`), hence `tail(text, 0)` in Python returns everything, not nothing.

### Borrowing the lines

Returning `Vec<&str>` hands back *views* into the input, no copying. The borrow checker guarantees those slices can't outlive the text they point into.

---


## Your Task

Implement both:

- `head(text: &str, n: usize) -> Vec<&str>` → the first `n` lines.
- `tail(text: &str, n: usize) -> Vec<&str>` → the last `n` lines.

Fewer than `n` lines available → return all of them. `n == 0` → empty `Vec` (for both).

---


## Example

assert_eq!(head("a\nb\nc\nd", 2), vec!["a", "b"]);
assert_eq!(tail("a\nb\nc\nd", 2), vec!["c", "d"]);
assert_eq!(tail("a\nb", 5), vec!["a", "b"]);


---


## Further Reading


- [Iterator::take](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take) — yield at most n items
- [usize::saturating_sub](https://doc.rust-lang.org/std/primitive.usize.html#method.saturating_sub) — subtract without underflow
- [str::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines) — iterate over lines

