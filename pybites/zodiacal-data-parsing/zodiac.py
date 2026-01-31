import json
import os
from datetime import datetime
from pathlib import Path
from urllib.request import urlretrieve

import pytest
from zodiac import get_sign_by_date, get_sign_with_most_famous_people, get_signs, signs_are_mutually_compatible

# original source: https://zodiacal.herokuapp.com/api
URL = "https://bites-data.s3.us-east-2.amazonaws.com/zodiac.json"
TMP = os.getenv("TMP", "/tmp")
PATH = Path(TMP, "zodiac.json")


@pytest.fixture(scope="module")
def signs():
    if not PATH.exists():
        urlretrieve(URL, PATH)
    with open(PATH) as f:
        data = json.loads(f.read())
    return get_signs(data)


@pytest.fixture(scope="module")
def aries(signs):
    return signs[0]


@pytest.fixture(scope="module")
def leo(signs):
    return signs[4]


@pytest.fixture(scope="module")
def scorpio(signs):
    return signs[7]


@pytest.fixture(scope="module")
def sagittarius(signs):
    return signs[8]


def test_get_sign_with_most_famous_people(signs, scorpio):
    result = get_sign_with_most_famous_people(signs)
    assert result[0] == scorpio.name
    assert result[1] == 35


def test_sign_compatability_true(signs, aries, leo, sagittarius):
    assert signs_are_mutually_compatible(signs, aries.name, leo.name)
    assert signs_are_mutually_compatible(signs, leo.name, aries.name)
    assert signs_are_mutually_compatible(signs, aries.name, sagittarius.name)

    # Added leading space for errors in data
    assert signs_are_mutually_compatible(signs, leo.name, f" {sagittarius.name}")
    assert signs_are_mutually_compatible(signs, sagittarius.name, f" {leo.name}")


def test_sign_compatability_false(signs, aries, leo, scorpio):
    assert not signs_are_mutually_compatible(signs, aries.name, scorpio.name)
    assert not signs_are_mutually_compatible(signs, scorpio.name, aries.name)
    assert not signs_are_mutually_compatible(signs, scorpio.name, sagittarius.name)
    assert not signs_are_mutually_compatible(signs, leo.name, scorpio.name)


def test_get_sign_by_date(signs, aries, leo, sagittarius, scorpio):
    assert get_sign_by_date(signs, datetime(year=2000, month=3, day=21)) == aries.name
    assert get_sign_by_date(signs, datetime(year=2000, month=4, day=3)) == aries.name
    assert get_sign_by_date(signs, datetime(year=2000, month=4, day=19)) == aries.name

    assert get_sign_by_date(signs, datetime(year=2000, month=7, day=23)) == leo.name
    assert get_sign_by_date(signs, datetime(year=2000, month=8, day=6)) == leo.name
    assert get_sign_by_date(signs, datetime(year=2000, month=8, day=22)) == leo.name

    assert get_sign_by_date(signs, datetime(year=2000, month=11, day=22)) == sagittarius.name
    assert get_sign_by_date(signs, datetime(year=2000, month=12, day=8)) == sagittarius.name
    assert get_sign_by_date(signs, datetime(year=2000, month=12, day=21)) == sagittarius.name

    assert get_sign_by_date(signs, datetime(year=2000, month=10, day=23)) == scorpio.name
    assert get_sign_by_date(signs, datetime(year=2000, month=11, day=5)) == scorpio.name
    assert get_sign_by_date(signs, datetime(year=2000, month=11, day=21)) == scorpio.name
