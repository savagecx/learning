import string
from itertools import chain, cycle


def sequence_generator():
    yield from cycle(chain.from_iterable(enumerate(string.ascii_uppercase, start=1)))
