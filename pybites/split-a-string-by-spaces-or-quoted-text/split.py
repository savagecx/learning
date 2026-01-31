import re


def split_words_and_quoted_text(text: str):
    """Split string text by space unless it is
    wrapped inside double quotes, returning a list
    of the elements.

    For example
    if text =
    'Should give "3 elements only"'

    the resulting list would be:
    ['Should', 'give', '3 elements only']
    """
    return [word for word in re.split(r'"([\w\s]+)"|\s', text) if word]
    # Reference solution uses shlex.split(text)
