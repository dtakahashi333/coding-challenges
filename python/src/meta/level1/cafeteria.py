#! /bin/env python3

from typing import List
import math


def getMaxAdditionalDinersCount(N: int, K: int, M: int, S: List[int]) -> int:
    diners = sorted(S)
    # first
    additional_diners: int = (diners[0] - 0 - 1) // (K + 1)
    previous: int = diners[0]
    for i in range(1, M, 1):
        additional_diners += (diners[i] - previous - 1 - K) // (K + 1)
        previous = diners[i]
    # last
    additional_diners += (N - previous) // (K + 1)

    return additional_diners
