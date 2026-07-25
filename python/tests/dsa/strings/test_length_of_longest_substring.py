#!/usr/bin/env python3

# python/tests/dsa/strings/test_length_of_longest_substring.py

from unittest import TestCase
from dsa.strings.length_of_longest_substring import Solution


class TestLengthOfLongestSubstring(TestCase):
    def test_length_of_longest_substring1(self):
        s = "abcabcbb"
        sol = Solution()
        self.assertEqual(sol.lengthOfLongestSubstring(s), 3)
        self.assertEqual(sol.lengthOfLongestSubstring2(s), 3)

    def test_length_of_longest_substring2(self):
        s = "bbbbb"
        sol = Solution()
        self.assertEqual(sol.lengthOfLongestSubstring(s), 1)
        self.assertEqual(sol.lengthOfLongestSubstring2(s), 1)

    def test_length_of_longest_substring3(self):
        s = "pwwkew"
        sol = Solution()
        self.assertEqual(sol.lengthOfLongestSubstring(s), 3)
        self.assertEqual(sol.lengthOfLongestSubstring2(s), 3)
