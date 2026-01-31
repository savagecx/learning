def convert(value: float, fmt: str) -> float:
    """Converts the value to the designated format.

    :param value: The value to be converted must be numeric or raise a TypeError
    :param fmt: String indicating format to convert to
    :return: Float rounded to 4 decimal places after conversion
    """
    if not isinstance(value, (int, float)):
        raise TypeError

    CM_PER_IN = 2.54
    IN_PER_CM = 0.3937008

    if fmt.lower() == "cm":
        return round(value * CM_PER_IN, 4)
    elif fmt.lower() == "in":
        return round(value * IN_PER_CM, 4)
    else:
        raise ValueError
