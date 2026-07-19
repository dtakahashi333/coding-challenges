#!/usr/bin/env python3

# python/tests/dsa/arrays/test_max_area.py

from unittest import TestCase
from dsa.arrays.max_area import Solution


class TestMaxArea(TestCase):
    def test_max_area1(self):
        height = [1, 8, 6, 2, 5, 4, 8, 3, 7]
        s = Solution()
        self.assertEqual(s.maxArea(height), 49)
        self.assertEqual(s.maxArea2(height), 49)

    def test_max_area2(self):
        height = [1, 1]
        s = Solution()
        self.assertEqual(s.maxArea(height), 1)
        self.assertEqual(s.maxArea2(height), 1)
