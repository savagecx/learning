import pytest
from account import Account


def test_account_simple():
    acc = Account("bob")

    assert acc.owner == "bob"
    assert acc.amount == 0
    assert repr(acc) == "Account('bob', 0)"
    assert str(acc) == "Account of bob with starting amount: 0"


def test_account_with_amount():
    acc = Account("tim", 10)

    assert acc.owner == "tim"
    assert acc.amount == 10
    assert repr(acc) == "Account('tim', 10)"
    assert str(acc) == "Account of tim with starting amount: 10"


def test_account_with_transactions():
    acc = Account("jo", 10)
    assert acc.amount == 10
    assert acc.balance == 10
    assert len(acc) == 0

    acc.add_transaction(15)
    assert acc.amount == 10
    assert acc.balance == 25
    assert len(acc) == 1
    assert acc[0] == 15


def test_account_comparison():
    acc_bob = Account("bob")
    acc_tim = Account("tim", 100)
    assert acc_tim > acc_bob

    acc_bob.add_transaction(100)
    assert acc_tim == acc_bob


def test_account_negative():
    acc = Account("jo", -50)
    acc.amount = -50
    acc.balance == -50

    acc.add_transaction(-50)
    acc.amount = -50
    acc.balance == -100

    acc.add_transaction(150)
    acc.amount = -50
    acc.balance == 50


def test_add_account_str():
    acc = Account("tim")
    with pytest.raises(ValueError):
        acc.add_transaction("1000")


def test_add_accounts():
    acc_bob = Account("bob")
    acc_bob.add_transaction(10)
    acc_tim = Account("tim", 100)

    acc_both = acc_tim + acc_bob
    assert acc_both.owner == "tim&bob"
    assert acc_both.amount == 100
    assert acc_both.balance == 110
    assert len(acc_both) == 1
    assert acc_both[0] == 10
    assert repr(acc_both) == "Account('tim&bob', 100)"
    assert str(acc_both) == "Account of tim&bob with starting amount: 100"
