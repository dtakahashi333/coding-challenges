#!/usr/bin/env python3

# python/src/dsa/arrays/two_sum_2.py

# 167. Two Sum II - Input Array Is Sorted
# https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

from typing import List


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        head = 0
        tail = len(numbers) - 1
        while head < tail:
            current_sum = numbers[head] + numbers[tail]
            if current_sum == target:
                return [head + 1, tail + 1]
            elif current_sum < target:
                head += 1
            else:
                tail -= 1

        raise ValueError("No two sum solution found")
