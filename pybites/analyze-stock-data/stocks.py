from collections import Counter

import requests

STOCK_DATA = "https://bites-data.s3.us-east-2.amazonaws.com/stocks.json"

# pre-work: load JSON data into program

with requests.Session() as s:
    data = s.get(STOCK_DATA).json()


def _cap_str_to_mln_float(cap: str) -> float:
    """If cap = 'n/a' return 0, else:
    - strip off leading '$',
    - if 'M' in cap value, strip it off and return value as float,
    - if 'B', strip it off, multiply by 1,000 and return
      value as float"""
    if cap.lower() == "n/a":
        return 0
    elif cap.endswith("M"):
        return float(cap.strip("$M"))
    elif cap.endswith("B"):
        return float(cap.strip("$B")) * 1000

    return float(cap.lstrip("$"))


def get_industry_cap(industry: str) -> float:
    """Return the sum of all cap values for given industry, use
    the _cap_str_to_mln_float to parse the cap values,
    return a float with 2 digit precision"""
    return round(
        sum(_cap_str_to_mln_float(stock["cap"]) for stock in data if stock["industry"].lower() == industry.lower()),
        2,
    )


def get_stock_symbol_with_highest_cap():
    """Return the stock symbol (e.g. PACD) with the highest cap, use
    the _cap_str_to_mln_float to parse the cap values"""
    top = max(data, key=lambda stock: _cap_str_to_mln_float(stock["cap"]))
    return top["symbol"]


def get_sectors_with_max_and_min_stocks():
    """Return a tuple of the sectors with most and least stocks,
    discard n/a"""
    count = Counter(stock["sector"] for stock in data if stock["sector"].lower() != "n/a").most_common()
    return count[0][0], count[-1][0]
