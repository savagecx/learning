import os
from collections import defaultdict
from datetime import date
from urllib.request import urlretrieve

from bs4 import BeautifulSoup, element

# prep data
tmp = os.getenv("TMP", "/tmp")
page = "us_holidays.html"
holidays_page = os.path.join(tmp, page)
urlretrieve(f"https://bites-data.s3.us-east-2.amazonaws.com/{page}", holidays_page)

with open(holidays_page) as f:
    content = f.read()


def get_us_bank_holidays(content: str = content) -> defaultdict[str, list]:
    """Receive scraped html output, make a BS object, parse the bank
    holiday table (css class = list-table), and return a dict of
    keys -> months and values -> list of bank holidays"""
    holidays = defaultdict(list)
    soup = BeautifulSoup(content, "html.parser")

    for row in soup.find("table", class_="list-table").tbody:
        if type(row) is element.Tag:
            month = "{:02d}".format(date.fromisoformat(row.time.string).month)
            holiday = row.a.string.strip()
            holidays[month].append(holiday)
    return holidays
