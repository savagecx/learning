from itertools import zip_longest
from textwrap import wrap

COL_WIDTH = 20
COL_PAD = 5


def text_to_columns(text: str) -> str:
    """Split text (input arg) to columns, the amount of double
    newlines (\n\n) in text determines the amount of columns.
    Return a string with the column output like:
    line1\nline2\nline3\n ... etc ...
    See also the tests for more info."""
    lines = [tuple(wrap(line, COL_WIDTH)) for line in text.splitlines() if line]
    columnized = ""
    for line in zip_longest(*lines, fillvalue=""):
        row = ""
        for column in line:
            row += f"{column: <{COL_WIDTH + COL_PAD}}"
        columnized += row.rstrip()
    return columnized
