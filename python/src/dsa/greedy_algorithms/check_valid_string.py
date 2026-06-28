#!/bin/env python3

# python/src/dsa/greedy_algorithms/check_valid_string.py

# 678. Valid Parenthesis String
# https://leetcode.com/problems/valid-parenthesis-string/description/


class Solution:
    def checkValidString(self, s: str) -> bool:
        if s[0] == "(":
            lefts = [1, 1]
        elif s[0] == "*":
            lefts = [0, 1]
        else:
            return False

        for c in s[1:]:
            if c == "(":
                lefts = [x + 1 for x in lefts]
            elif c == ")":
                if lefts[1] > 0:
                    lefts = [max(0, x - 1) for x in lefts]
                else:
                    return False
            elif c == "*":
                lefts[0] = max(0, lefts[0] - 1)
                lefts[1] += 1

        return lefts[0] == 0
