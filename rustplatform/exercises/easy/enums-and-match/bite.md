# Enums and Match

- Level: easy
- Author: Pybites

## Instructions
🎯 Python's `enum` module provides enumeration types:

```python
from enum import Enum

class Direction(Enum):
    NORTH = "north"
    SOUTH = "south"
    EAST = "east"
    WEST = "west"

def opposite(d: Direction) -> Direction:
    match d:
        case Direction.NORTH: return Direction.SOUTH
        case Direction.SOUTH: return Direction.NORTH
        case Direction.EAST:  return Direction.WEST
        case Direction.WEST:  return Direction.EAST
```

Rust's enums look similar but are more powerful — they can carry data inside variants (we'll see that in later exercises). For now, let's start with the simple case:

```rust
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
```


### match with enums

Rust's `match` pairs naturally with enums. The compiler **requires exhaustive matching** — you must handle every variant:

```rust
fn clothing(season: Season) -> &'static str {
    match season {
        Season::Spring => "light jacket",
        Season::Summer => "shorts",
        Season::Autumn => "sweater",
        Season::Winter => "heavy coat",
    }
}
```

If you add a new variant to the enum later, the compiler will flag every `match` that doesn't handle it. Python's structural `match` doesn't have this guarantee.

### &'static str

The return type `&'static str` means "a string slice that lives for the entire program." String literals like `"stop"` are compiled into the binary, so they're always available. Think of it as a constant string reference — the Rust equivalent of returning a Python string literal.

---


## Your Task


1. Define a `TrafficLight` enum with variants: `Red`, `Yellow`, `Green`
2. Implement `action(light: TrafficLight) -> &'static str`:
3. `Red` → `"stop"`
4. `Yellow` → `"slow down"`
5. `Green` → `"go"`


---


## Example


```rust
assert_eq!(action(TrafficLight::Red), "stop");
assert_eq!(action(TrafficLight::Yellow), "slow down");
assert_eq!(action(TrafficLight::Green), "go");
```


---


> **Dive deeper:** In [our Rust Developer Cohort](https://scriptertorust.com/), you'll define a `Token` enum with variants like `LeftBrace`, `String(String)`, `Number(f64)` — turning raw JSON text into structured, type-safe data.


---


## Further Reading


- [The Rust Book — Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) — enum syntax and variants
- [The Rust Book — The match Control Flow](https://doc.rust-lang.org/book/ch06-02-match.html) — exhaustive pattern matching

