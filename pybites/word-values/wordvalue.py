import os
import urllib.request

# PREWORK
TMP = os.getenv("TMP", "/tmp")
S3 = "https://bites-data.s3.us-east-2.amazonaws.com/"
DICT = "dictionary.txt"
DICTIONARY = os.path.join(TMP, DICT)
urllib.request.urlretrieve(f"{S3}{DICT}", DICTIONARY)

scrabble_scores = [
    (1, "E A O I N R T L S U"),
    (2, "D G"),
    (3, "B C M P"),
    (4, "F H V W Y"),
    (5, "K"),
    (8, "J X"),
    (10, "Q Z"),
]
LETTER_SCORES = {letter: score for score, letters in scrabble_scores for letter in letters.split()}


def load_words() -> list[str]:
    with open(file=DICTIONARY, mode="r", encoding="utf-8") as file:
        return [line.strip() for line in file.readlines()]


def calc_word_value(word) -> int:
    return sum([LETTER_SCORES.get(char, 0) for char in word.upper()])


def max_word_value(words):
    return max(words, key=calc_word_value)
