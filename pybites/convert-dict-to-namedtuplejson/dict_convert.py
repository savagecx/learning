import json
from collections import namedtuple
from datetime import datetime

blog = dict(
    name="PyBites",
    founders=("Julian", "Bob"),
    started=datetime(year=2016, month=12, day=19),
    tags=["Python", "Code Challenges", "Learn by Doing"],
    location="Spain/Australia",
    site="https://pybit.es",
)

Blog = namedtuple(typename="Blog", field_names=blog.keys())


def dict2nt(dict_: dict) -> Blog:
    return Blog(**dict_)


def nt2json(nt: Blog) -> str:
    return json.dumps(nt._asdict(), default=str)
