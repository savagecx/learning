PYBITES = "pybites"


def convert_pybites_chars(text: str):
    """Swap case all characters in the word pybites for the given text.
    Return the resulting string."""
    converted = ""
    for c in text:
        if c.lower() in PYBITES:
            c = c.swapcase()
        converted += c

    return converted
