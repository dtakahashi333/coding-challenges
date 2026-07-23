#!/usr/bin/env python3

# python/src/dsa/arrays/max_profit.py

# 121. Best Time to Buy and Sell Stock
# https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/

from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        length = len(prices)
        max_profit = 0
        for i in range(length):
            for j in range(i + 1, length, 1):
                max_profit = max(max_profit, prices[j] - prices[i])

        return max_profit

    def maxProfit2(self, prices: List[int]) -> int:
        min_price = prices[0]
        max_profit = 0
        for i in range(1, len(prices), 1):
            max_profit = max(max_profit, prices[i] - min_price)
            min_price = min(min_price, prices[i])

        return max_profit
