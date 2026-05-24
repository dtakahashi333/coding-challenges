#!/bin/env python3

# python/src/dsa/arrays/third_max.py

from typing import List


class Solution:
    def thirdMax(self, nums: List[int]) -> int:
        first = nums[0]
        second = None
        third = None
        for i in range(1, len(nums)):
            if nums[i] > first:
                third = second
                second = first
                first = nums[i]
            elif nums[i] != first and (second is None or nums[i] > second):
                third = second
                second = nums[i]
            elif (
                nums[i] != first
                and nums[i] != second
                and (third is None or nums[i] > third)
            ):
                third = nums[i]

        return first if third is None else third
