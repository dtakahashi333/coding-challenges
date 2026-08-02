#!/usr/bin/env python3

# python/src/dsa/binary_search/find_min.py

# 153. Find Minimum in Rotated Sorted Array
# https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/

from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        length = len(nums)
        first, last = 0, length - 1
        while first < last:
            middle = first + (last - first) // 2
            if nums[middle] > nums[last]:
                first = middle + 1
            else:
                last = middle

        return nums[first]
