#!/usr/bin/env python3

# python/tests/dsa/arrays/test_sort_colors.py

from unittest import TestCase
from dsa.arrays.sort_colors import Solution


class TestSortColors(TestCase):
    def test_sort_colors1(self):
        nums = [2, 0, 2, 1, 1, 0]
        s = Solution()
        s.sortColors(nums)
        self.assertListEqual(nums, [0, 0, 1, 1, 2, 2])
        s.sortColors2(nums)
        self.assertListEqual(nums, [0, 0, 1, 1, 2, 2])

    def test_sort_colors2(self):
        nums = [2, 0, 1]
        s = Solution()
        s.sortColors(nums)
        self.assertListEqual(nums, [0, 1, 2])
        s.sortColors2(nums)
        self.assertListEqual(nums, [0, 1, 2])
