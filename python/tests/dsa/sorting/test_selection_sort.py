#!/bin/env python3

# python/tests/dsa/sorting/test_merge_sort.py

from unittest import TestCase
from dsa.sorting.selection_sort import Solution


class TestSelectionSort(TestCase):
    def test_selection_sort1(self):
        nums = [4, 2, 1, 3]
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [1, 2, 3, 4])

    def test_selection_sort2(self):
        nums = [-1, 5, 3, 4, 0]
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [-1, 0, 3, 4, 5])

    def test_selection_sort3(self):
        nums = []
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [])

    def test_selection_sort4(self):
        nums = [1]
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [1])

    def test_selection_sort5(self):
        nums = [2, 1]
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [1, 2])

    def test_selection_sort6(self):
        nums = [1, 1, 1]
        s = Solution()
        s.selectionSort(nums)
        self.assertListEqual(nums, [1, 1, 1])
