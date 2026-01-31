from typing import Generator, List

IGNORE_CHAR = "b"
QUIT_CHAR = "q"
MAX_NAMES = 5


def filter_names(names: List[str]) -> Generator:
    count = 0
    for name in names:
        if name.startswith(QUIT_CHAR) or count >= MAX_NAMES:
            break
        elif name.startswith(IGNORE_CHAR) or any(c.isdigit() for c in name):
            continue

        count += 1
        yield name
