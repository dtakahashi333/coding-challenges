#!/bin/env python3

# python/src/dsa/greedy_algorithms/lemonade_change.py

# 860. Lemonade Change
# https://leetcode.com/problems/lemonade-change/description/

from typing import List


class Solution:
    def lemonadeChange(self, bills: List[int]) -> bool:
        counts = {5: 0, 10: 0, 20: 0}

        for bill in bills:
            original = bill
            while bill > 0:
                if bill == 5:
                    bill -= 5
                elif bill == 10:
                    if counts[5] > 0:
                        counts[5] -= 1
                        bill -= 10
                    else:
                        return False
                else:
                    if counts[10] > 0:
                        counts[10] -= 1
                        bill -= 10
                    elif counts[5] > 0:
                        counts[5] -= 1
                        bill -= 5
                    else:
                        return False

            counts[original] += 1

        return True
