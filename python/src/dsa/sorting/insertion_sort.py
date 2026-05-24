#!/bin/env python3

# python/src/dsa/sorting/insertion_sort.py

from typing import List


class Solution:
    def insertionSort(self, nums: List[int]) -> List[int]:
        size = len(nums)
        for i in range(0, size):
            for j in range(0, i + 1):
                cur = nums[i]
                if cur < nums[j]:
                    for k in range(i, j, -1):
                        nums[k] = nums[k - 1]
                    nums[j] = cur
                    break

        return nums
