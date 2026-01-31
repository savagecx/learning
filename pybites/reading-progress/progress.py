from datetime import datetime
from typing import Optional

DAYS_IN_YEAR = 365


def ontrack_reading(books_goal: int, books_read: int, day_of_year: Optional[int] = None) -> bool:
    if books_goal == 0:
        return True

    if books_read == 0:
        return False

    if not day_of_year:
        day_of_year = datetime.now().timetuple().tm_yday

    return (DAYS_IN_YEAR / books_goal) >= (day_of_year / books_read)
