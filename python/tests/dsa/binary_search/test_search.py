#!/usr/bin/env python3

# python/tests/dsa/binary_search/test_search.py

from unittest import TestCase
from dsa.binary_search.search import Solution


class TestSearch(TestCase):
    def test_search1(self):
        nums = [-1, 0, 3, 5, 9, 12]
        target = 9
        s = Solution()
        self.assertEqual(s.search(nums, target), 4)

    def test_search2(self):
        nums = [-1, 0, 3, 5, 9, 12]
        target = 2
        s = Solution()
        self.assertEqual(s.search(nums, target), -1)
