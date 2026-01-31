from typing import List, Union


def join_lists(lst_of_lst: List[List[str]], sep: str) -> Union[List[str], None]:
    if not lst_of_lst:
        return None

    joined = []
    for lst in lst_of_lst:
        joined.append(sep)
        joined.extend(lst)
    return joined[1:]
