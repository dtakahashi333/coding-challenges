#!/bin/env python3

# python/src/dsa/divide_and_conquer/is_winning_hand.py

from typing import List


class Solution:
    def is_winning_hand(self, nums: List[int]) -> bool:
        if len(nums) != 14:
            return False

        map = {}

        for n in nums:
            if n not in map:
                map[n] = 0
            map[n] += 1

        for n, c in map.items():
            if c >= 2:
                map[n] -= 2
                if self.helper(map):
                    return True
                map[n] += 2

        return False

    def helper(self, map: dict[int, int]) -> bool:
        active_tiles = [k for (k, v) in map.items() if v > 0]

        if len(active_tiles) == 0:
            return True

        # Get an active tile with the smallest number
        n = min(active_tiles)
        c = map[n]

        if c >= 3:
            map[n] -= 3
            if self.helper(map):
                return True
            map[n] += 3

        if c >= 1 and map.get(n + 1, 0) >= 1 and map.get(n + 2, 0) >= 1:
            map[n] -= 1
            map[n + 1] -= 1
            map[n + 2] -= 1
            if self.helper(map):
                return True
            map[n] += 1
            map[n + 1] += 1
            map[n + 2] += 1

        return False
