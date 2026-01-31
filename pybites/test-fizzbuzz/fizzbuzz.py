from fizzbuzz import fizzbuzz

# write one or more pytest functions below, they need to start with test_


def test_zero():
    assert fizzbuzz(0) == "Fizz Buzz"


def test_div_by_three():
    assert fizzbuzz(3) == "Fizz"
    assert fizzbuzz(21) == "Fizz"
    assert fizzbuzz(126) == "Fizz"

    assert fizzbuzz(-3) == "Fizz"
    assert fizzbuzz(-21) == "Fizz"
    assert fizzbuzz(-126) == "Fizz"


def test_div_by_five():
    assert fizzbuzz(5) == "Buzz"
    assert fizzbuzz(50) == "Buzz"
    assert fizzbuzz(125) == "Buzz"

    assert fizzbuzz(-5) == "Buzz"
    assert fizzbuzz(-50) == "Buzz"
    assert fizzbuzz(-125) == "Buzz"


def test_div_by_three_and_five():
    assert fizzbuzz(15) == "Fizz Buzz"
    assert fizzbuzz(45) == "Fizz Buzz"
    assert fizzbuzz(300) == "Fizz Buzz"

    assert fizzbuzz(-15) == "Fizz Buzz"
    assert fizzbuzz(-45) == "Fizz Buzz"
    assert fizzbuzz(-300) == "Fizz Buzz"


def test_not_div_by_three_or_five():
    assert fizzbuzz(1) == 1
    assert fizzbuzz(13) == 13
    assert fizzbuzz(202) == 202

    assert fizzbuzz(-1) == -1
    assert fizzbuzz(-13) == -13
    assert fizzbuzz(-202) == -202
