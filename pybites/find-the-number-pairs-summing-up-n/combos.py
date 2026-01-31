from itertools import combinations


def find_number_pairs(numbers, N=10) -> list[tuple]:
    return [pair for pair in combinations(numbers, 2) if sum(pair) == N]
