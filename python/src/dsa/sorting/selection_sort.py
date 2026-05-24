#!/bin/env python3

# python/src/dsa/sorting/selection_sort.py

from typing import List


class Solution:
    def selectionSort(self, nums: List[int]):
        size = len(nums)
        for i in range(0, size):
            min = (2**63) - 1
            for j in range(i, size):
                if nums[j] < min:
                    min = nums[j]
                    nums[j] = nums[i]
                    nums[i] = min

        return nums
