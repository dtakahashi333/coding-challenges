#!/bin/env python3

# python/tests/dsa/divide_and_conquer/test_count_good_numbers.py

from unittest import TestCase
from dsa.divide_and_conquer.count_good_numbers import Solution


class TestCountGoodNumbers(TestCase):
    def test_count_good_numbers1(self):
        n = 1
        s = Solution()
        self.assertEqual(s.countGoodNumbers(n), 5)
        self.assertEqual(s.countGoodNumbers2(n), 5)

    def test_count_good_numbers2(self):
        n = 4
        s = Solution()
        self.assertEqual(s.countGoodNumbers(n), 400)
        self.assertEqual(s.countGoodNumbers2(n), 400)

    def test_count_good_numbers3(self):
        n = 50
        s = Solution()
        self.assertEqual(s.countGoodNumbers(n), 564908303)
        self.assertEqual(s.countGoodNumbers2(n), 564908303)

    def test_count_good_numbers4(self):
        n = 806166225460393
        s = Solution()
        # self.assertEqual(s.countGoodNumbers(n), 643535977)
        self.assertEqual(s.countGoodNumbers2(n), 643535977)
