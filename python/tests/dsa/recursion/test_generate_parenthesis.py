#!/bin/env python3

# python/tests/dsa/recursive/test_generate_parenthesis.py

from unittest import TestCase
from dsa.recursion.generate_parenthesis import Solution


class TestGenerateParenthesis(TestCase):
    def test_generate_parenthesis1(self):
        n = 3
        s = Solution()
        result = s.generateParenthesis(n)
        self.assertListEqual(
            sorted(result), sorted(["((()))", "(()())", "(())()", "()(())", "()()()"])
        )

    def test_generate_parenthesis2(self):
        n = 1
        s = Solution()
        result = s.generateParenthesis(n)
        self.assertListEqual(sorted(result), sorted(["()"]))
