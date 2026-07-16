#!/usr/bin/env python3

# python/src/dsa/arrays/contains_duplicate.py

# 217. Contains Duplicate
# https://leetcode.com/problems/contains-duplicate/description/

from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        seen = set()
        for num in nums:
            if num in seen:
                return True

            seen.add(num)

        return False
