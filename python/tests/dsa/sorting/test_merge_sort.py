#!/bin/env python3

# python/tests/dsa/sorting/test_merge_sort.py

from unittest import TestCase
from dsa.sorting.merge_sort import Solution


class TestMergeSort(TestCase):
    def test_merge_sort1(self):
        nums = [4, 2, 1, 3]
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [1, 2, 3, 4])

    def test_merge_sort2(self):
        nums = [-1, 5, 3, 4, 0]
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [-1, 0, 3, 4, 5])

    def test_merge_sort3(self):
        nums = []
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [])

    def test_merge_sort4(self):
        nums = [1]
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [1])

    def test_merge_sort5(self):
        nums = [2, 1]
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [1, 2])

    def test_merge_sort6(self):
        nums = [1, 1, 1]
        s = Solution()
        s.mergeSort(nums)
        self.assertListEqual(nums, [1, 1, 1])
