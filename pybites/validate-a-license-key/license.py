import re


def validate_license(key: str) -> bool:
    """Write a regex that matches a PyBites license key
    (e.g. PB-U8N435EH-PG65PW87-IXPWQG5T-898XSZI4)
    """
    exp = r"PB(-[A-Z0-9]{8}){4}"
    return bool(re.fullmatch(exp, key))
