#!/usr/bin/env python3

# python/tests/dsa/binary_search/test_find_min.py

from unittest import TestCase
from dsa.binary_search.find_min import Solution


class TestFindMin(TestCase):
    def test_find_min1(self):
        nums = [3, 4, 5, 1, 2]
        s = Solution()
        self.assertEqual(s.findMin(nums), 1)

    def test_find_min2(self):
        nums = [4, 5, 6, 7, 0, 1, 2]
        s = Solution()
        self.assertEqual(s.findMin(nums), 0)

    def test_find_min3(self):
        nums = [11, 13, 15, 17]
        s = Solution()
        self.assertEqual(s.findMin(nums), 11)

    def test_find_min4(self):
        nums = [2, 1]
        s = Solution()
        self.assertEqual(s.findMin(nums), 1)

    def test_find_min5(self):
        nums = [4, 5, 1, 2, 3]
        s = Solution()
        self.assertEqual(s.findMin(nums), 1)

    def test_find_min6(self):
        nums = [3, 1, 2]
        s = Solution()
        self.assertEqual(s.findMin(nums), 1)

    def test_find_min7(self):
        nums = [5, 1, 2, 3, 4]
        s = Solution()
        self.assertEqual(s.findMin(nums), 1)
