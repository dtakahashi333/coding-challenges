#!/usr/bin/env python3

# python/src/dsa/binary_search/search_insert.py

# 35. Search Insert Position
# https://leetcode.com/problems/search-insert-position/description/

from typing import List


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        length = len(nums)
        first, last = 0, length - 1
        while first < last:
            middle = first + (last - first) // 2
            num = nums[middle]
            if num == target:
                return middle
            elif num < target:
                first = middle + 1
            else:
                last = middle - 1

        # first == last
        num = nums[first]
        if num == target:
            return first
        elif num < target:
            return first + 1
        else:
            return first

    def searchInsert2(self, nums: List[int], target: int) -> int:
        first, last = 0, len(nums) - 1

        while first <= last:
            middle = first + (last - first) // 2

            if nums[middle] == target:
                return middle
            elif nums[middle] < target:
                first = middle + 1
            else:
                last = middle - 1

        return first
