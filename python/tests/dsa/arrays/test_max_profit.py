#!/usr/bin/env python3

# python/tests/dsa/arrays/test_max_profit.py

from unittest import TestCase
from dsa.arrays.max_profit import Solution


class TestMaxProfit(TestCase):
    def test_max_profit1(self):
        prices = [7, 1, 5, 3, 6, 4]
        s = Solution()
        self.assertEqual(s.maxProfit(prices), 5)
        self.assertEqual(s.maxProfit2(prices), 5)

    def test_max_profit2(self):
        prices = [7, 6, 4, 3, 1]
        s = Solution()
        self.assertEqual(s.maxProfit(prices), 0)
        self.assertEqual(s.maxProfit2(prices), 0)
