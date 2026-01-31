import base64
import csv


def get_credit_cards(data: bytes) -> list[str]:
    """Decode the base64 encoded data which gives you a csv
    of "first_name,last_name,credit_card", from which you have
    to extract the credit card numbers.
    """
    return [
        row.get("credit_card")
        for row in csv.DictReader(str(base64.b64decode(data), encoding="utf8").splitlines())
    ]
