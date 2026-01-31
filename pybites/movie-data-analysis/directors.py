import csv
import os
import statistics
from collections import defaultdict, namedtuple
from urllib.request import urlretrieve

BASE_URL = "https://bites-data.s3.us-east-2.amazonaws.com/"
TMP = os.getenv("TMP", "/tmp")

fname = "movie_metadata.csv"
remote = os.path.join(BASE_URL, fname)
local = os.path.join(TMP, fname)
urlretrieve(remote, local)

MOVIE_DATA = local
MIN_MOVIES = 4
MIN_YEAR = 1960

Movie = namedtuple("Movie", "title year score")


def get_movies_by_director() -> dict[str, list[Movie]]:
    """Extracts all movies from csv and stores them in a dict,
    where keys are directors, and values are a list of movies,
    use the defined Movie namedtuple"""

    movies_by_director = defaultdict(list)
    with open(file=MOVIE_DATA, mode="r", encoding="utf8") as file:
        for row in csv.DictReader(file):
            title, year, score = row.get("movie_title"), row.get("title_year"), row.get("imdb_score")
            if not all((title, year, score)) or int(year) < MIN_YEAR:
                continue
            movie = Movie(title.strip(), int(year), float(score))
            movies_by_director[row["director_name"]].append(movie)
    return movies_by_director


def calc_mean_score(movies: list[Movie]) -> float:
    """Helper method to calculate mean of list of Movie namedtuples,
    round the mean to 1 decimal place"""
    return round(statistics.fmean([movie.score for movie in movies]), 1)


def get_average_scores(directors: dict[str, list[Movie]]) -> list[tuple[str, float]]:
    """Iterate through the directors dict (returned by get_movies_by_director),
    return a list of tuples (director, average_score) ordered by highest
    score in descending order. Only take directors into account
    with >= MIN_MOVIES"""
    return sorted(
        [
            (
                director,
                calc_mean_score(movies),
            )
            for director, movies in directors.items()
            if len(movies) >= MIN_MOVIES
        ],
        key=lambda x: x[1],
        reverse=True,
    )
