def is_armstrong(n: int) -> bool:
    total = sum([int(d) ** len(str(n)) for d in str(n)])
    return total == n
