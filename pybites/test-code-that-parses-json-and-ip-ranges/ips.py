import os
from ipaddress import IPv4Network
from pathlib import Path
from urllib.request import urlretrieve

import pytest
from test_ips import ServiceIPRange, get_aws_service_range, parse_ipv4_service_ranges

URL = "https://bites-data.s3.us-east-2.amazonaws.com/ip-ranges.json"
TMP = os.getenv("TMP", "/tmp")
PATH = Path(TMP, "ip-ranges.json")
IP = IPv4Network("192.0.2.8/29")


@pytest.fixture(scope="module")
def json_file():
    """Import data into tmp folder"""
    urlretrieve(URL, PATH)
    return PATH


def test_valid_path(json_file):
    ranges = parse_ipv4_service_ranges(json_file)
    assert (
        ServiceIPRange(
            service="AMAZON",
            region="eu-west-1",
            cidr=IPv4Network("13.248.118.0/24"),
        )
        == ranges[0]
    )

    assert (
        ServiceIPRange(
            service="AMAZON",
            region="eu-west-1",
            cidr=IPv4Network("63.34.60.0/22"),
        )
        == ranges[1800]
    )


def test_invalid_path():
    with pytest.raises(AttributeError):
        parse_ipv4_service_ranges("invalid")


def test_no_path():
    with pytest.raises(TypeError):
        parse_ipv4_service_ranges()


def test_valid_get_aws_service_range(json_file):
    ranges = parse_ipv4_service_ranges(json_file)
    assert 3 == len(get_aws_service_range(address="99.79.34.0", service_ranges=ranges))


def test_invalid_ip_get_aws_service_range(json_file):
    with pytest.raises(ValueError):
        get_aws_service_range(address="99.79.34.0.5", service_ranges=[])


def test_missing_args_get_aws_service_range(json_file):
    with pytest.raises(TypeError):
        get_aws_service_range(service_ranges=[])

    with pytest.raises(TypeError):
        get_aws_service_range(address="99.79.34.0")
