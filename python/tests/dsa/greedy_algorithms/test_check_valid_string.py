#!/bin/env python3

# python/tests/dsa/greedy_algorithms/test_check_valid_string.py

from unittest import TestCase
from dsa.greedy_algorithms.check_valid_string import Solution


class TestCheckValidString(TestCase):
    def test_check_valid_string1(self):
        s = "()"
        sol = Solution()
        result = sol.checkValidString(s)
        self.assertTrue(result)

    def test_check_valid_string2(self):
        s = "(*)"
        sol = Solution()
        result = sol.checkValidString(s)
        self.assertTrue(result)

    def test_check_valid_string3(self):
        s = "(*))"
        sol = Solution()
        result = sol.checkValidString(s)
        self.assertTrue(result)

    def test_check_valid_string4(self):
        s = "((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()"
        sol = Solution()
        result = sol.checkValidString(s)
        self.assertTrue(result)
