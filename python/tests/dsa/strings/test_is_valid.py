#!/usr/bin/env python3

# python/tests/dsa/strings/is_valid.py

from unittest import TestCase
from dsa.strings.is_valid import Solution


class TestIsValid(TestCase):
    def test_is_valid1(self):
        s = "()"
        sol = Solution()
        self.assertTrue(sol.isValid(s))

    def test_is_valid2(self):
        s = "()[]{}"
        sol = Solution()
        self.assertTrue(sol.isValid(s))

    def test_is_valid3(self):
        s = "(]"
        sol = Solution()
        self.assertFalse(sol.isValid(s))

    def test_is_valid4(self):
        s = "([])"
        sol = Solution()
        self.assertTrue(sol.isValid(s))

    def test_is_valid5(self):
        s = "([)]"
        sol = Solution()
        self.assertFalse(sol.isValid(s))
