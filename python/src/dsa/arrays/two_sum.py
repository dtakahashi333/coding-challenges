#!/usr/bin/env python3

# python/src/dsa/arrays/two_sum.py

# 1. Two Sum
# https://leetcode.com/problems/two-sum/description/

from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        pairs = list(enumerate(nums))  # index, value pairs

        pairs.sort(key=lambda x: x[1])
        head, tail = 0, len(pairs) - 1
        while head < tail:
            current_sum = pairs[head][1] + pairs[tail][1]
            if target == current_sum:
                return [pairs[head][0], pairs[tail][0]]
            elif target < current_sum:
                tail -= 1
            else:
                head += 1

        raise ValueError("No two sum solution found")

    def twoSum2(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i, num in enumerate(nums):
            complement = target - num
            if complement in seen:
                return [seen[complement], i]

            seen[num] = i

        raise ValueError("No two sum solution found")
