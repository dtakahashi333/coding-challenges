#!/bin/env python3

# python/tests/dsa/greedy_algorithms/test_lemonade_change.py

from unittest import TestCase
from dsa.greedy_algorithms.lemonade_change import Solution


class TestLemonadeChange(TestCase):
    def test_lemonade_change1(self):
        bills = [5, 5, 5, 10, 20]
        s = Solution()
        self.assertTrue(s.lemonadeChange(bills))
        # self.assertTrue(s.lemonadeChange2(bills))

    def test_lemonade_change2(self):
        bills = [5, 5, 10, 10, 20]
        s = Solution()
        self.assertFalse(s.lemonadeChange(bills))
        # self.assertFalse(s.lemonadeChange2(bills))
