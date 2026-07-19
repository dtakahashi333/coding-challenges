#!/usr/bin/env python3

# python/tests/dsa/strings/test_is_palindrome.py

from unittest import TestCase
from dsa.strings.is_palindrome import Solution


class TestIsPalindrome(TestCase):
    def test_is_palindrome1(self):
        s = "A man, a plan, a canal: Panama"
        sol = Solution()
        self.assertTrue(sol.isPalindrome(s))

    def test_is_palindrome2(self):
        s = "race a car"
        sol = Solution()
        self.assertFalse(sol.isPalindrome(s))

    def test_is_palindrome3(self):
        s = " "
        sol = Solution()
        self.assertTrue(sol.isPalindrome(s))
