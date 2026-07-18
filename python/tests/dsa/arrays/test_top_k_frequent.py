#!/usr/bin/env python3

# python/tests/dsa/arrays/test_top_k_frequent.py

from unittest import TestCase
from dsa.arrays.top_k_frequent import Solution


class TestTopKFrequent(TestCase):
    def test_top_k_frequent1(self):
        nums = [1, 1, 1, 2, 2, 3]
        k = 2
        s = Solution()
        self.assertListEqual(sorted(s.topKFrequent(nums, k)), sorted([1, 2]))
        self.assertListEqual(sorted(s.topKFrequent2(nums, k)), sorted([1, 2]))

    def test_top_k_frequent2(self):
        nums = [1]
        k = 1
        s = Solution()
        self.assertListEqual(sorted(s.topKFrequent(nums, k)), sorted([1]))
        self.assertListEqual(sorted(s.topKFrequent2(nums, k)), sorted([1]))

    def test_top_k_frequent3(self):
        nums = [1, 2, 1, 2, 1, 2, 3, 1, 3, 2]
        k = 2
        s = Solution()
        self.assertListEqual(sorted(s.topKFrequent(nums, k)), sorted([1, 2]))
        self.assertListEqual(sorted(s.topKFrequent2(nums, k)), sorted([1, 2]))
