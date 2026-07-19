#!/usr/bin/env python3

# python/src/dsa/arrays/longest_consecutive.py

# 128. Longest Consecutive Sequence
# https://leetcode.com/problems/longest-consecutive-sequence/

from typing import List


class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        seen = set()
        for num in nums:
            seen.add(num)

        longest = 0
        counted = set()
        for num in seen:
            if num in counted:
                continue

            count = 1
            x = num - 1
            while x in seen:
                count += 1
                counted.add(x)
                x -= 1

            x = num + 1
            while x in seen:
                count += 1
                counted.add(x)
                x += 1

            longest = max(longest, count)
            counted.add(num)

        return longest

    def longestConsecutive2(self, nums: List[int]) -> int:
        seen = set()
        for num in nums:
            seen.add(num)

        beginning = []
        for num in seen:
            if num - 1 not in seen:
                beginning.append(num)

        longest = 0
        for num in beginning:
            count = 0
            while num in seen:
                count += 1
                num += 1
            longest = max(longest, count)

        return longest
