from decimal import Decimal
from typing import List, TypeVar

T = TypeVar("T", int, float)


def n_digit_numbers(numbers: List[T], n: int) -> List[int]:
    if n < 1:
        raise ValueError

    new_numbers = []

    for num in numbers:
        if len(str(int(num))) >= n:
            new_numbers.append(int(str(num)[:n]))
        else:
            new_numbers.append(int(Decimal(str(num)).shift(n - 1)))

    return new_numbers
