def round_to_next(number: int, multiple: int):
    if not (remainder := number % multiple):
        return number

    number += multiple - remainder
    return number
