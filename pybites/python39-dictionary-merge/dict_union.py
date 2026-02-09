def combine_and_count(a: dict, b: dict) -> dict:
    """Combine two dictionaries.

    Return  new dictionary with the combined keys and values.
    For any key found in both dictionaries,
    return the sum of the values for that key.

    Args:
      a: The first dictionary.
      b: The second dictionary.

    Returns:
      A dictionary with the contents of both.
      Values of any shared keys are summed.
    """
    if not isinstance(a, dict):
        raise TypeError("Parameter 'a' is not a dictionary.")
    elif not isinstance(b, dict):
        raise TypeError("Parameter 'b' is not a dictionary.")

    for key, val in a.items():
        if b.get(key) is not None:
            b[key] += val

    return a | b

    # Pybites Solution
    # combo = a | b
    # for key in a.keys() & b.keys():
    #     combo[key] += a[key]  # without this, combo[key] == b[key]
    # return combo
