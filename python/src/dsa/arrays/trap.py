#!/usr/bin/env python3

# python/src/dsa/arrays/trap.py

# 42. Trapping Rain Water
# https://leetcode.com/problems/trapping-rain-water/description/

from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        length = len(height)
        if length <= 2:
            return 0
        total_water = 0
        for i in range(1, length - 1, 1):
            # find left wall
            left_wall = height[i]
            for j in range(i - 1, -1, -1):
                left_wall = max(left_wall, height[j])
            right_wall = height[i]
            for j in range(i + 1, length, 1):
                right_wall = max(right_wall, height[j])

            total_water += max(min(left_wall, right_wall) - height[i], 0)

        return total_water

    def trap2(self, height: List[int]) -> int:
        length = len(height)
        if length <= 2:
            return 0
        left_max = [height[0]] * length
        for i in range(1, length, 1):
            left_max[i] = max(left_max[i - 1], height[i - 1])
        right_max = [height[length - 1]] * length
        for i in range(length - 2, -1, -1):
            right_max[i] = max(right_max[i + 1], height[i + 1])
        total_water = 0
        for i in range(1, length - 1, 1):
            total_water += max(0, min(left_max[i], right_max[i]) - height[i])

        return total_water

    def trap3(self, height: List[int]) -> int:
        length = len(height)
        if length <= 2:
            return 0
        left, right = 0, length - 1
        left_max, right_max = height[left], height[right]
        total_water = 0
        while left < right:
            if left_max < right_max:
                total_water += left_max - height[left]
                left += 1
                left_max = max(left_max, height[left])
            else:
                total_water += right_max - height[right]
                right -= 1
                right_max = max(right_max, height[right])

        return total_water

    def trap4(self, height: List[int]) -> int:
        length = len(height)
        if length <= 2:
            return 0
        stack = []
        total_water = 0
        for i in range(length):
            if stack and height[stack[-1]] < height[i]:
                right_index = i
                right = height[right_index]
                while stack and right >= height[stack[-1]]:
                    bottom_index = stack.pop()
                    bottom = height[bottom_index]
                    if stack:
                        left_index = stack[-1]
                        left = height[left_index]
                        total_water += (min(left, right) - bottom) * (
                            right_index - left_index - 1
                        )
            stack.append(i)

        return total_water
