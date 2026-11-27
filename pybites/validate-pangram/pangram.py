import string


def validate_pangram(sentence: str) -> bool:
    """Check if a sentence is a pangram
    A pangram is a sentence containing every letter in the English alphabet.
    The input `sentence` should be a string containing only English letters.
    The function returns True if the sentence is a pangram, and False otherwise.
    """
    return set(sentence.lower().replace(" ", "")) == set(string.ascii_lowercase)


if __name__ == "__main__":
    sentence = "The quick brown fox jumps over a lazy dog"
    validate_pangram(sentence)
