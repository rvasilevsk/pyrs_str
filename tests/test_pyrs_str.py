import pytest
from pyrs_str import *


@pytest.mark.parametrize(
    "left, right",
    [
        ("", ""),
        (" \r\n\t ", ""),
        ("  aaa   bbb  ", "aaa bbb"),
    ]
)
def test_str_simplify(left, right) -> None:
    assert str_simplify(left) == right
