#!/usr/bin/env python3

# python/tests/dsa/binary_search/test_rotated_sorted_array.py

from unittest import TestCase
from dsa.binary_search.rotated_sorted_array import Solution


class TestRotatedSortedArray(TestCase):
    def test_rotated_sorted_array1(self):
        nums = [4, 5, 6, 7, 0, 1, 2]
        target = 0
        s = Solution()
        self.assertEqual(s.search(nums, target), 4)

    def test_rotated_sorted_array2(self):
        nums = [4, 5, 6, 7, 0, 1, 2]
        target = 3
        s = Solution()
        self.assertEqual(s.search(nums, target), -1)

    def test_rotated_sorted_array3(self):
        nums = nums = [1]
        target = 0
        s = Solution()
        self.assertEqual(s.search(nums, target), -1)

    def test_rotated_sorted_array4(self):
        nums = nums = [3, 1]
        target = 1
        s = Solution()
        self.assertEqual(s.search(nums, target), 1)
