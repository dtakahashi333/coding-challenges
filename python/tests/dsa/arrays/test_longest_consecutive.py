#!/usr/bin/env python3

# python/tests/dsa/arrays/test_longest_consecutive.py

from unittest import TestCase
from dsa.arrays.longest_consecutive import Solution


class TestLongestConsecutive(TestCase):
    def test_longest_consecutive1(self):
        nums = [100, 4, 200, 1, 3, 2]
        s = Solution()
        self.assertEqual(s.longestConsecutive(nums), 4)
        self.assertEqual(s.longestConsecutive2(nums), 4)

    def test_longest_consecutive2(self):
        nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1]
        s = Solution()
        self.assertEqual(s.longestConsecutive(nums), 9)
        self.assertEqual(s.longestConsecutive2(nums), 9)

    def test_longest_consecutive3(self):
        nums = [1, 0, 1, 2]
        s = Solution()
        self.assertEqual(s.longestConsecutive(nums), 3)
        self.assertEqual(s.longestConsecutive2(nums), 3)
