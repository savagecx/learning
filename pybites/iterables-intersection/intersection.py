from typing import Any, Iterable, Set


def intersection(*args: Iterable) -> Set[Any]:
    iterables = [arg for arg in args if arg]
    if not iterables:
        return set()

    intersection = set(iterables[0])

    for obj in iterables:
        intersection &= set(obj)

    return intersection
