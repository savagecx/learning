def reverse_letters(string: str) -> str:
    """Reverse letters in a string but keep the order of the non-letters the same"""
    reversed = [ch for ch in string if ch.isalpha()]
    reversed.reverse()
    for i, ch in enumerate(string):
        if not ch.isalpha():
            reversed.insert(i, ch)
    return "".join(reversed)


if __name__ == "__main__":
    string = "ab5DEf"
    print(reverse_letters(string))
