import os
import statistics as st
from urllib.request import urlretrieve

TMP = os.getenv("TMP", "/tmp")
S3 = "https://bites-data.s3.us-east-2.amazonaws.com/"
DATA = "testfiles_number_loc.txt"
STATS = os.path.join(TMP, DATA)
if not os.path.isfile(STATS):
    urlretrieve(os.path.join(S3, DATA), STATS)

STATS_OUTPUT = """
Basic statistics:
- count     : {count:7d}
- min       : {min_:7d}
- max       : {max_:7d}
- mean      : {mean:7.2f}

Population variance:
- pstdev    : {pstdev:7.2f}
- pvariance : {pvariance:7.2f}

Estimated variance for sample:
- count     : {sample_count:7.2f}
- stdev     : {sample_stdev:7.2f}
- variance  : {sample_variance:7.2f}
"""


def get_all_line_counts(data: str = STATS) -> list:
    """Get all 186 line counts from the STATS file,
    returning a list of ints"""
    with open(data, "r") as f:
        return [int(line.split()[0]) for line in f.readlines()]


def create_stats_report(data=None):
    if data is None:
        # converting to a list in case a generator was returned
        data = list(get_all_line_counts())

    # taking a sample for the last section
    sample = list(data)[::2]

    stats = dict(
        count=len(data),
        min_=min(data),
        max_=max(data),
        mean=st.mean(data),
        pstdev=st.pstdev(data),
        pvariance=st.pvariance(data),
        sample_count=len(sample),
        sample_stdev=st.stdev(sample),
        sample_variance=st.variance(sample),
    )

    return STATS_OUTPUT.format(**stats)


print(create_stats_report())
