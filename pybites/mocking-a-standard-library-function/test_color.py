from unittest.mock import patch

import color
import pytest


@pytest.fixture(scope="module")
def gen():
    return color.gen_hex_color()


@patch("color.sample")
def test_gen_hex_color(mock_sample, gen):
    mock_sample.return_value = (
        122,
        250,
        16,
    )
    assert "#7AFA10" == next(gen)
