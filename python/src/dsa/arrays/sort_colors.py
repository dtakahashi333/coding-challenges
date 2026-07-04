#!/bin/env python3

# python/src/dsa/arrays/sort_colors.py

# 75. Sort Colors
# https://leetcode.com/problems/sort-colors/description/

from typing import List


class Solution:
    def sortColors(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        self.devideAndConquer(nums, 0, len(nums) - 1)

    def devideAndConquer(self, nums: List[int], start: int, end: int) -> None:
        if start >= end:
            return

        middle = start + (end - start) // 2

        self.devideAndConquer(nums, start, middle)
        self.devideAndConquer(nums, middle + 1, end)

        self.merge(nums, start, middle, middle + 1, end)
        print(nums)

    def merge(self, nums: List[int], s1: int, e1: int, s2: int, e2: int) -> None:
        while s1 <= e1 and s2 <= e2:
            if nums[s1] <= nums[s2]:
                s1 += 1
            else:
                i = s2
                while i > s1:
                    tmp = nums[i]
                    nums[i] = nums[i - 1]
                    nums[i - 1] = tmp
                    i -= 1
                s2 += 1
                s1 += 1
                e1 += 1

    def sortColors2(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        low = 0
        mid = 0
        high = len(nums) - 1
        while mid <= high:
            n = nums[mid]
            match n:
                case 0:
                    nums[low], nums[mid] = nums[mid], nums[low]
                    low += 1
                    mid += 1
                case 1:
                    mid += 1
                case 2:
                    nums[high], nums[mid] = nums[mid], nums[high]
                    high -= 1
