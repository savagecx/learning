from collections import Counter


def major_n_minor(numbers):
    """
    Input: an array with integer numbers
    Output: the majority and minority number
    """
    c = Counter(numbers)
    arranged = c.most_common()
    major, minor = int(arranged[0][0]), int(arranged[-1][0])

    return major, minor
