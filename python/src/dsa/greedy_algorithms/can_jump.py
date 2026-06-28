#!/bin/env python3

# python/src/dsa/greedy_algorithms/can_jump.py

# 55. Jump Game
# https://leetcode.com/problems/jump-game/description/

from typing import List


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        if len(nums) == 1 and nums[0] == 0:
            return True

        max_reach = 0
        i = 0
        while i <= min(len(nums) - 1, max_reach):
            max_reach = max(max_reach, i + nums[i])
            i += 1

        return max_reach >= len(nums) - 1
