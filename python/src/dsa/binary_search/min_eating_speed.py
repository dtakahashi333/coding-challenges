#!/usr/bin/env python3

# python/src/dsa/binary_search/min_eating_speed.py

# 875. Koko Eating Bananas
# https://leetcode.com/problems/koko-eating-bananas/description/

from typing import List
from math import ceil


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        min_speed, max_speed = 1, max(piles)
        while min_speed < max_speed:
            mid_speed = min_speed + (max_speed - min_speed) // 2
            total_hours = sum([int(ceil(x / mid_speed)) for x in piles])
            if total_hours <= h:
                max_speed = mid_speed
            else:
                min_speed = mid_speed + 1

        return min_speed
