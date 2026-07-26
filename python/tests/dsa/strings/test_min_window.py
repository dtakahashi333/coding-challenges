#!/usr/bin/env min_window.py

# python/tests/dsa/strings/test_min_window.py

from unittest import TestCase
from dsa.strings.min_window import Solution


class TestMinWindow(TestCase):
    def test_min_window1(self):
        s = "ADOBECODEBANC"
        t = "ABC"
        sol = Solution()
        self.assertEqual(sol.minWindow(s, t), "BANC")

    def test_min_window2(self):
        s = "a"
        t = "a"
        sol = Solution()
        self.assertEqual(sol.minWindow(s, t), "a")

    def test_min_window3(self):
        s = "a"
        t = "aa"
        sol = Solution()
        self.assertEqual(sol.minWindow(s, t), "")

    def test_min_window4(self):
        s = "baBBba"
        t = "aB"
        sol = Solution()
        self.assertEqual(sol.minWindow(s, t), "aB")
