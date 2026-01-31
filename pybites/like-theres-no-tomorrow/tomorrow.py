from datetime import datetime, timedelta, timezone


def tomorrow(today=None):
    if not today:
        today = datetime.now(tz=timezone.utc).date()

    return today + timedelta(days=1)
