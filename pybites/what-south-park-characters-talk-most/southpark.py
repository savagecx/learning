import csv
from collections import Counter, defaultdict

import requests

CSV_URL = "https://raw.githubusercontent.com/pybites/SouthParkData/master/by-season/Season-{}.csv"  # noqa E501


def get_season_csv_file(season: int) -> str:
    """Receives a season int, and downloads loads in its
    corresponding CSV_URL"""
    with requests.Session() as s:
        download = s.get(CSV_URL.format(season))
        return download.content.decode("utf-8")


def get_num_words_spoken_by_character_per_episode(content: str) -> defaultdict[str, Counter]:
    """Receives loaded csv content (str) and returns a dict of
    keys=characters and values=Counter object,
    which is a mapping of episode=>words spoken"""
    word_counts = defaultdict(Counter)
    for row in csv.DictReader(content.splitlines()):
        episode, character, line = row.get("Episode"), row.get("Character"), row.get("Line")
        word_counts[character][episode] += len(line.split())

    return word_counts
