#!/usr/bin/env python3

# python/src/dsa/binary_search/search.py

# 704. Binary Search
# https://leetcode.com/problems/binary-search/description/

from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        first, last = 0, len(nums) - 1
        while first <= last:
            middle = first + (last - first) // 2
            if nums[middle] == target:
                return middle
            elif nums[middle] < target:
                first = middle + 1
            else:
                last = middle - 1

        return -1  # not found
