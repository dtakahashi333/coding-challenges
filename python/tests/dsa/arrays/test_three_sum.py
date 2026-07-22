#!/usr/bin/env python3

# python/tests/dsa/arrays/test_three_sum.py

from unittest import TestCase
from dsa.arrays.three_sum import Solution


class TestThreeSum(TestCase):
    def test_three_sum1(self):
        nums = [-1, 0, 1, 2, -1, -4]
        s = Solution()
        self.assertListEqual(
            sorted([sorted(x) for x in s.threeSum(nums)]),
            sorted([sorted(x) for x in [[-1, -1, 2], [-1, 0, 1]]]),
        )
        self.assertListEqual(
            sorted([sorted(x) for x in s.threeSum2(nums)]),
            sorted([sorted(x) for x in [[-1, -1, 2], [-1, 0, 1]]]),
        )
        self.assertListEqual(
            sorted([sorted(x) for x in s.threeSum3(nums)]),
            sorted([sorted(x) for x in [[-1, -1, 2], [-1, 0, 1]]]),
        )

    def test_three_sum2(self):
        nums = [0, 1, 1]
        s = Solution()
        self.assertListEqual([sorted(x) for x in s.threeSum(nums)], [])
        self.assertListEqual([sorted(x) for x in s.threeSum2(nums)], [])
        self.assertListEqual([sorted(x) for x in s.threeSum3(nums)], [])

    def test_three_sum3(self):
        nums = [0, 0, 0]
        s = Solution()
        self.assertListEqual([sorted(x) for x in s.threeSum(nums)], [[0, 0, 0]])
        self.assertListEqual([sorted(x) for x in s.threeSum2(nums)], [[0, 0, 0]])
        self.assertListEqual([sorted(x) for x in s.threeSum3(nums)], [[0, 0, 0]])
