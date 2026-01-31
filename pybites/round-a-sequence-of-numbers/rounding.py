from math import ceil, floor


def round_up_or_down(transactions: list[float], up: bool = True):
    """Round the list of transactions passed in.
    If up=True (default) round up, else round down.
    Return a new list of rounded values
    """
    if up:
        return [ceil(row) for row in transactions]

    return [floor(row) for row in transactions]
