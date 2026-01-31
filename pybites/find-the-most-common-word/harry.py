import os
import string
import urllib.request
from collections import Counter
from typing import Tuple

# data provided
tmp = os.getenv("TMP", "/tmp")
stopwords_file = os.path.join(tmp, "stopwords")
harry_text = os.path.join(tmp, "harry")
urllib.request.urlretrieve("https://bites-data.s3.us-east-2.amazonaws.com/stopwords.txt", stopwords_file)
urllib.request.urlretrieve("https://bites-data.s3.us-east-2.amazonaws.com/harry.txt", harry_text)


def get_harry_most_common_word() -> Tuple[str, int]:
    with open(file=stopwords_file, mode="r", encoding="utf8") as sw_file:
        # Use read() and split() to remove the \n from each word
        stopwords = set(sw_file.read().lower().split())

    strip_ch = string.whitespace + string.punctuation
    word_counts = Counter()

    with open(file=harry_text, mode="r", encoding="utf8") as h_file:
        for word in h_file.read().split():
            word = word.strip(strip_ch).lower()
            if word not in stopwords and word.isalnum():
                word_counts[word] += 1

    return word_counts.most_common(1)[0]
