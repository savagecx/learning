import re


def get_users(passwd: str) -> dict:
    """Split password output by newline,
    extract user and name (1st and 5th columns),
    strip trailing commas from name,
    replace multiple commas in name with a single space
    return dict of keys = user, values = name.
    """
    users = {}
    for line in passwd.strip().splitlines():
        parts = line.split(":")
        username = parts[0]
        name = parts[4] or "unknown"
        users[username] = re.sub(r",+", " ", name).rstrip()

    return users
