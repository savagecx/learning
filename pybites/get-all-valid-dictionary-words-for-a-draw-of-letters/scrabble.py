import os
import urllib.request
from itertools import permutations
from types import GeneratorType

# PREWORK
TMP = os.getenv("TMP", "/tmp")
DICT = "dictionary.txt"
DICTIONARY = os.path.join(TMP, DICT)
urllib.request.urlretrieve(f"https://bites-data.s3.us-east-2.amazonaws.com/{DICT}", DICTIONARY)

with open(DICTIONARY) as f:
    dictionary = set([word.strip().lower() for word in f.read().split()])


def get_possible_dict_words(draw: list) -> set:
    """Get all possible words from a draw (list of letters) which are
    valid dictionary words. Use _get_permutations_draw and provided
    dictionary"""
    perms = _get_permutations_draw([letter.lower() for letter in draw])
    return set(["".join(letters) for letters in perms]) & dictionary


def _get_permutations_draw(draw: list[str]) -> GeneratorType[tuple]:
    """Helper to get all permutations of a draw (list of letters), hint:
    use itertools.permutations (order of letters matters)"""
    for i in range(1, len(draw) + 1):
        yield from permutations(draw, i)
