from unittest.mock import patch

import pytest
from guess import GuessGame, InvalidNumber

exceptiondata = [
    (
        "str",
        "Not a number",
    ),
    (
        -1,
        "Negative number",
    ),
    (20, "Number too high"),
]


@pytest.mark.parametrize("input,error", exceptiondata)
def test_constructor_validation(input, error):
    with pytest.raises(InvalidNumber, match=error):
        GuessGame(input)


guessdata = [
    (
        5,
        (5,),
        (
            "Guess a number: ",
            "You guessed it!",
            "",
        ),
    ),
    (
        5,
        (
            4,
            5,
        ),
        (
            "Guess a number: ",
            "Too low",
            "Guess a number: ",
            "You guessed it!",
            "",
        ),
    ),
    (
        5,
        (
            7,
            5,
        ),
        (
            "Guess a number: ",
            "Too high",
            "Guess a number: ",
            "You guessed it!",
            "",
        ),
    ),
    (
        2,
        (
            7,
            6,
        ),
        (
            "Guess a number: ",
            "Too high",
            "Guess a number: ",
            "Too high",
            "Sorry, the number was 5",
            "",
        ),
    ),
]


@patch("guess.input")
@pytest.mark.parametrize("guesses,input,expected", guessdata)
def test_game(mock_input, capsys, guesses, input, expected):
    mock_input.side_effect = input
    game = GuessGame(5, guesses)
    game()
    captured = capsys.readouterr()
    assert captured.out == "\n".join(expected)


@patch("guess.input")
def test_str_guess(mock_input, capsys):
    mock_input.side_effect = [1, "str", 5]
    game = GuessGame(5)
    game()
    captured = capsys.readouterr()
    assert captured.out == "\n".join(
        [
            "Guess a number: ",
            "Too low",
            "Guess a number: ",
            "Enter a number, try again",
            "Guess a number: ",
            "You guessed it!",
            "",
        ]
    )
