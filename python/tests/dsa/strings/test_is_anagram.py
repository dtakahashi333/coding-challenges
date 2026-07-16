#!/usr/bin/env python3

# python/tests/dsa/strings/test_is_anagram.py

from unittest import TestCase
from dsa.strings.is_anagram import Solution


class TestIsAnagram(TestCase):
    def test_is_anagram1(self):
        s = "anagram"
        t = "nagaram"
        sol = Solution()
        self.assertTrue(sol.isAnagram(s, t))

    def test_is_anagram2(self):
        s = "rat"
        t = "car"
        sol = Solution()
        self.assertFalse(sol.isAnagram(s, t))
