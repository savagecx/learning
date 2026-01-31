import os
import random
import string

import pytest
from movies import MovieDb

salt = "".join(random.choice(string.ascii_lowercase) for i in range(20))
DB = os.path.join(os.getenv("TMP", "/tmp"), f"movies_{salt}.db")
# https://www.imdb.com/list/ls055592025/
DATA = [
    ("The Godfather", 1972, 9.2),
    ("The Shawshank Redemption", 1994, 9.3),
    ("Schindler's List", 1993, 8.9),
    ("Raging Bull", 1980, 8.2),
    ("Casablanca", 1942, 8.5),
    ("Citizen Kane", 1941, 8.3),
    ("Gone with the Wind", 1939, 8.1),
    ("The Wizard of Oz", 1939, 8),
    ("One Flew Over the Cuckoo's Nest", 1975, 8.7),
    ("Lawrence of Arabia", 1962, 8.3),
]
TABLE = "movies"


@pytest.fixture
def db():
    movies = MovieDb(DB, DATA, TABLE)
    movies.init()
    yield movies

    movies.drop_table()


def test_movie_add(db):
    rowid = db.add("Transformers", 2007, 7.1)
    assert rowid == 11


def test_movie_delete(db):
    result = db.query(*DATA[3])
    assert len(result) == 1
    assert result[0][1:] == DATA[3]

    db.delete(4)
    result = db.query(*DATA[3])
    assert len(result) == 0


@pytest.mark.parametrize("title, year, score", DATA)
def test_movie_query_single_result(db, title, year, score):
    result = db.query(title, year, score)
    assert len(result) == 1
    assert result[0][1:] == (title, year, score)


def test_movie_query_multi_result(db):
    year_query = db.query(year=1939)
    assert len(year_query) == 2
    assert year_query[0][1:] == DATA[6]
    assert year_query[1][1:] == DATA[7]

    score_query = db.query(score_gt=9)
    assert len(score_query) == 2
    assert score_query[0][1:] == DATA[0]
    assert score_query[1][1:] == DATA[1]


def test_movie_query_no_result(db):
    title_query = db.query(title="Transformers")
    assert len(title_query) == 0

    year_query = db.query(year=2005)
    assert len(year_query) == 0

    score_query = db.query(score_gt=9.5)
    assert len(score_query) == 0
