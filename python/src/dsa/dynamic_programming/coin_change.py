#!/bin/env python3

# python/src/dsa/dynamic_programming/coin_change.py

# 322. Coin Change
# https://leetcode.com/problems/coin-change/description/

from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        coins.sort(reverse=True)
        dp = {}
        return self.coinChangeHelper(coins, amount, dp)

    def coinChangeHelper(
        self, coins: List[int], amount: int, dp: dict[int, int]
    ) -> int:
        if amount == 0:
            return 0
        elif amount < 0:
            return -1

        if amount in dp:
            return dp[amount]

        min_count = float("inf")
        for coin in coins:
            count = self.coinChangeHelper(coins, amount - coin, dp)

            if count >= 0:
                min_count = min(min_count, count)

        dp[amount] = -1 if min_count == float("inf") else min_count + 1

        return dp[amount]
