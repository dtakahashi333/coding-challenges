#!/usr/bin/env python3

# python/src/dsa/strings/is_valid.py

# 20. Valid Parentheses
# https://leetcode.com/problems/valid-parentheses/


class Solution:
    def isValid(self, s: str) -> bool:
        pairs = {"(": ")", "{": "}", "[": "]"}
        stack = []
        for c in s:
            if c in pairs:
                stack.append(c)
            else:
                if not stack or c != pairs[stack.pop()]:
                    return False

        return not stack
