import pprint
from typing import Any


def pretty_string(obj: Any) -> str:
    return pprint.pformat(object=obj, width=60, depth=2, sort_dicts=True)
