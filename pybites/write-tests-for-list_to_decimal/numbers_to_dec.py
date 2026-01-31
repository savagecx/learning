import pytest
from numbers_to_dec import list_to_decimal


def test_out_of_range():
    with pytest.raises(ValueError):
        list_to_decimal([1, 5, 15])

    with pytest.raises(ValueError):
        list_to_decimal([3, -7, 1])


def test_non_int():
    with pytest.raises(TypeError):
        list_to_decimal([1, True, 6])

    with pytest.raises(TypeError):
        list_to_decimal([4, 7, "foo"])


def test_leading_zero():
    assert list_to_decimal([0, 6, 3, 9]) == 639


def test_correctly_four_ints():
    assert list_to_decimal([2, 8, 0, 1]) == 2801
