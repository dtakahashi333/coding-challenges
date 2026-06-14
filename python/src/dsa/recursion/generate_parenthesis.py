#!/bin/env python3

# python/src/dsa/recursion/generate_parehthesis.py

# 22. Generate Parentheses
# https://leetcode.com/problems/generate-parentheses/description/

from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        res = []
        self.generateParenthesisHelper(n * 2 - 1, "(", res, (n - 1, n))
        return res

    def generateParenthesisHelper(
        self, n: int, paren: str, res: List[str], paren_count
    ):
        if n == 0:
            res.append(paren)
        if paren_count[0] > 0:
            self.generateParenthesisHelper(
                n - 1, paren + "(", res, (paren_count[0] - 1, paren_count[1])
            )
        if paren_count[1] > 0 and paren_count[0] < paren_count[1]:
            self.generateParenthesisHelper(
                n - 1, paren + ")", res, (paren_count[0], paren_count[1] - 1)
            )
