# top_words: Pipeline Capstone

- Level: hard
- Author: Pybites

## Instructions
I love shell one-liners that combine different Unix tools 🎯 - for example, to find the most frequent words in a file:

```
$ cat file
air
boat
crane
air
boat
boat
$ tr -s ' ' '\n' < file | sort | uniq -c | sort -rn | head

   3 boat
   2 air
   1 crane
```

You've now built the Rust equivalent of every stage: splitting, counting, sorting. This little capstone exercise composes them into one function: count word frequencies and return the top `n`. In Python you'd use `Counter`:
from collections import Counter
def top_words(text: str, n: int) -> list[tuple[str, int]]:
    counts = Counter(text.lower().split())
    return counts.most_common(n)


### Count with the entry API

Rust has no `Counter`, but the `HashMap` entry API tallies in one line per item: `entry` hands you the slot for a key, `or_insert` seeds it on first sight, and you bump it. You might already used this in the [collections track](../../../../track/collections/). Example:
use std::collections::HashMap;
let mut counts: HashMap<char, usize> = HashMap::new();
for c in "banana".chars() {
    *counts.entry(c).or_insert(0) += 1;   // {'b':1, 'a':3, 'n':2}
}


### Sort the pairs, with a tie-breaker

`most_common` sorts by count, descending. Turn the map into a `Vec` of pairs and sort it. Two refinements over a plain sort:

- **Descending:** a comparator returns the ordering of its two arguments; swap which side you compare to flip the direction.
- **Stable ties:** when the primary keys are equal, fall back to a second key so the output is deterministic. `Ordering::then` chains comparisons — the second is consulted only when the first is a tie. With neutral data:

use std::cmp::Ordering;
let mut v = vec![("bb", 1), ("a", 1), ("c", 5)];
v.sort_by(|x, y| ... );
// by .1 descending; ties broken by .0 ascending

 

---


## Your Task

Implement `top_words(text: &str, n: usize) -> Vec<(String, usize)>`. Split on whitespace, lowercase, count, and return the top `n` as `(word, count)`; most frequent first, ties broken alphabetically.

---


## Example

assert_eq!(
    top_words("the cat the dog the", 2),
    vec![("the".to_string(), 3), ("cat".to_string(), 1)]
);


---


## Further Reading


- [HashMap::entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) — count in one line
- [Ordering::then](https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then) — chain comparisons for tie-breaking
- [slice::sort_by](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by) — sort with a custom comparator
- [Vec::truncate](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate) — keep only the first n elements

