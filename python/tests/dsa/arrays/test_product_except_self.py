#!/usr/bin/env python3

# python/tests/dsa/arrays/test_product_except_self.py

from unittest import TestCase
from dsa.arrays.product_except_self import Solution


class TestProductExceptSelf(TestCase):
    def test_product_except_self1(self):
        nums = [1, 2, 3, 4]
        s = Solution()
        self.assertListEqual(s.productExceptSelf(nums), [24, 12, 8, 6])
        self.assertListEqual(s.productExceptSelf2(nums), [24, 12, 8, 6])

    def test_product_except_self2(self):
        nums = [-1, 1, 0, -3, 3]
        s = Solution()
        self.assertListEqual(s.productExceptSelf(nums), [0, 0, 9, 0, 0])
        self.assertListEqual(s.productExceptSelf2(nums), [0, 0, 9, 0, 0])
