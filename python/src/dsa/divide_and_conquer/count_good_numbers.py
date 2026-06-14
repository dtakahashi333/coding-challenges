#!/bin/env python3

# python/src/dsa/divide_and_conquer/count_good_numbers.py

# 1922. Count Good Numbers
# https://leetcode.com/problems/count-good-numbers/description/

class Solution:
    def countGoodNumbers(self, n: int) -> int:
        return self.countGoodNumbersHelper(0, n - 1)

    def countGoodNumbersHelper(self, start: int, end: int) -> int:
        MOD = 10 ** 9 + 7
        if start == end:
            if start % 2 == 0:  # even
                return 5
            else:  # odd
                return 4
        middle = start + (end - start) // 2
        return self.countGoodNumbersHelper(start, middle) \
            * self.countGoodNumbersHelper(middle + 1, end) % MOD

    def countGoodNumbers2(self, n: int) -> int:
        MOD = 10 ** 9 + 7
        odd_count = n // 2
        even_count = (n + 1) // 2
        return pow(5, even_count, MOD) * pow(4, odd_count, MOD) % MOD
