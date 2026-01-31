import pytest

from fibonacci import fib

# write one or more pytest functions below, they need to start with test_


def test_input_negative():
    with pytest.raises(ValueError):
        fib(-1)


def test_input_zero():
    assert fib(0) == 0


def test_input_one():
    assert fib(1) == 1


def test_input_sixth_element():
    assert fib(6) == 8
