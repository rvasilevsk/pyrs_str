import pytest
from pyrs_str import *


@pytest.mark.parametrize(
    "l, r",
    [
        ("", ""),
        (" \r\n\t ", ""),
        ("  aaa   bbb  ", "aaa bbb"),
    ]
)
def test_str_simplify(l, r) -> None:
    assert str_simplify(l) == r
