#!/usr/bin/env python3

# python/tests/dsa/arrays/test_two_sum.py

from unittest import TestCase
from dsa.arrays.two_sum_2 import Solution


class TestTwoSum2(TestCase):
    def test_two_sum1(self):
        nums = [2, 7, 11, 15]
        target = 9
        s = Solution()
        self.assertListEqual(sorted(s.twoSum(nums, target)), sorted([1, 2]))

    def test_two_sum2(self):
        nums = [2, 3, 4]
        target = 6
        s = Solution()
        self.assertListEqual(sorted(s.twoSum(nums, target)), sorted([1, 3]))

    def test_two_sum3(self):
        nums = [-1, 0]
        target = -1
        s = Solution()
        self.assertListEqual(sorted(s.twoSum(nums, target)), sorted([1, 2]))
