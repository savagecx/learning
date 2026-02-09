import os
import re
from collections import Counter
from urllib.request import urlretrieve

from dateutil.parser import parse

commits = os.path.join(os.getenv("TMP", "/tmp"), "commits")
urlretrieve("https://bites-data.s3.us-east-2.amazonaws.com/git_log_stat.out", commits)

# you can use this constant as key to the yyyymm:count dict
YEAR_MONTH = "{y}-{m:02d}"


def get_min_max_amount_of_commits(commit_log: str = commits, year: int | None = None) -> tuple[str, str]:
    """
    Calculate the amount of inserts / deletes per month from the
    provided commit log.

    Takes optional year arg, if provided only look at lines for
    that year, if not, use the entire file.

    Returns a tuple of (least_active_month, most_active_month)
    """
    log_format = r"^Date:\s+(\w{3} \w{3} \d+ \d{2}:\d{2}:\d{2} \d{4} [\+|\-]\d{4}) \|(?: \d+ files? changed)?(?:, (\d+) insertion\S{3,4})?(?:, (\d+) deletion\S{3,4})?$"
    count = Counter()
    with open(file=commit_log, mode="r", encoding="utf8") as commits:
        for commit in commits:
            commit_date, inserts, deletes = re.findall(log_format, commit)[0]
            commit_date = parse(commit_date)
            if year and commit_date.year != year:
                continue

            inserts = int(inserts) if inserts else 0
            deletes = int(deletes) if deletes else 0
            key = YEAR_MONTH.format(y=commit_date.year, m=commit_date.month)
            count[key] += inserts + deletes

    ordered_count = count.most_common()
    return ordered_count[-1][0], ordered_count[0][0]


if __name__ == "__main__":
    get_min_max_amount_of_commits()
