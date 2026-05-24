#!/bin/env python3

# python/tests/dsa/arrays/test_third_max.py

from unittest import TestCase
from dsa.arrays.third_max import Solution


class TestThirdMax(TestCase):
    def test_third_max1(self):
        nums = [3, 2, 1]
        s = Solution()
        res = s.thirdMax(nums)
        self.assertEqual(res, 1)

    def test_third_max2(self):
        nums = [1, 2]
        s = Solution()
        res = s.thirdMax(nums)
        self.assertEqual(res, 2)

    def test_third_max1(self):
        nums = [2, 2, 3, 1]
        s = Solution()
        res = s.thirdMax(nums)
        self.assertEqual(res, 1)
