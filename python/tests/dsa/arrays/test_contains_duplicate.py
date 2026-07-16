#!/usr/bin/env python3

# python/tests/dsa/arrays/test_contains_duplicate.py

from unittest import TestCase
from dsa.arrays.contains_duplicate import Solution


class TestContainsDuplicate(TestCase):
    def test_contains_duplicate1(self):
        nums = [1, 2, 3, 1]
        s = Solution()
        self.assertTrue(s.containsDuplicate(nums))

    def test_contains_duplicate2(self):
        nums = [1, 2, 3, 4]
        s = Solution()
        self.assertFalse(s.containsDuplicate(nums))

    def test_contains_duplicate3(self):
        nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
        s = Solution()
        self.assertTrue(s.containsDuplicate(nums))
