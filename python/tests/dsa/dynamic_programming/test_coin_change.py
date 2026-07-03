#!/bin/env python3

# python/tests/dsa/dynamic_programming/test_coin_change.py

from unittest import TestCase
from dsa.dynamic_programming.coin_change import Solution


class TestCoinChange(TestCase):
    def test_coin_change1(self):
        coins = [1, 2, 5]
        amount = 11
        s = Solution()
        self.assertEqual(s.coinChange(coins, amount), 3)

    def test_coin_change2(self):
        coins = [2]
        amount = 3
        s = Solution()
        self.assertEqual(s.coinChange(coins, amount), -1)

    def test_coin_change3(self):
        coins = [1]
        amount = 0
        s = Solution()
        self.assertEqual(s.coinChange(coins, amount), 0)

    def test_coin_change4(self):
        coins = [1, 2, 5]
        amount = 100
        s = Solution()
        self.assertEqual(s.coinChange(coins, amount), 20)

    def test_coin_change5(self):
        coins = [186, 419, 83, 408]
        amount = 6249
        s = Solution()
        self.assertEqual(s.coinChange(coins, amount), 20)
