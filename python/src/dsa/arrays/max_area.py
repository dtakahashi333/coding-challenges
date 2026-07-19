#!/usr/bin/env python3

# python/src/dsa/arrays/max_area.py

# 11. Container With Most Water
# https://leetcode.com/problems/container-with-most-water/description/

from typing import List


class Solution:
    def maxArea(self, height: List[int]) -> int:
        max_area = 0
        for i in range(len(height)):
            for j in range(i + 1, len(height), 1):
                current_area = (j - i) * min(height[i], height[j])
                max_area = max(max_area, current_area)

        return max_area

    def maxArea2(self, height: List[int]) -> int:
        max_area = 0
        left = 0
        right = len(height) - 1
        while left < right:
            current_area = (right - left) * min(height[left], height[right])
            max_area = max(max_area, current_area)
            if height[left] < height[right]:
                left += 1
            else:
                right -= 1

        return max_area
