from unicodedata import decomposition


def filter_accents(text: str) -> list[str]:
    """Return a sequence of accented characters found in
    the passed in lowercased text string
    """
    return [ch for ch in text.lower() if decomposition(ch)]
