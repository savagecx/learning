from textwrap import wrap


def get_weekdays(calendar_output: str) -> dict[int, str]:
    """Receives a multiline Unix cal output and returns a mapping (dict) where
    keys are int days and values are the 2 letter weekdays (Su Mo Tu ...)"""
    lines = calendar_output.splitlines()
    weekdays = lines[1].split()
    mapping = {}
    for line in lines[2:]:
        for i, date in enumerate(wrap(line, 3, drop_whitespace=False)):
            if date := date.strip():
                mapping[int(date)] = weekdays[i]
    return mapping
