#!/usr/bin/env python3

# python3/tests/dsa/binary_search/test_search_insert.py
from unittest import TestCase
from dsa.binary_search.search_insert import Solution


class TestSearchInsert(TestCase):
    def test_search_insert1(self):
        nums = [1, 3, 5, 6]
        target = 5
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 2)
        self.assertEqual(s.searchInsert2(nums, target), 2)

    def test_search_insert2(self):
        nums = nums = [1, 3, 5, 6]
        target = 2
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 1)
        self.assertEqual(s.searchInsert2(nums, target), 1)

    def test_search_insert3(self):
        nums = [1, 3, 5, 6]
        target = 7
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 4)
        self.assertEqual(s.searchInsert2(nums, target), 4)

    def test_search_insert4(self):
        nums = [1, 3, 5, 6]
        target = 0
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 0)
        self.assertEqual(s.searchInsert2(nums, target), 0)

    def test_search_insert5(self):
        nums = [1, 3]
        target = 2
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 1)
        self.assertEqual(s.searchInsert2(nums, target), 1)

    def test_search_insert6(self):
        nums = [1, 3, 5]
        target = 4
        s = Solution()
        self.assertEqual(s.searchInsert(nums, target), 2)
        self.assertEqual(s.searchInsert2(nums, target), 2)
