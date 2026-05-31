#!/bin/env python3

from unittest import TestCase

from dsa.divide_and_conquer.is_winning_hand import Solution


class TestIsWinningHand(TestCase):
    def test_is_winning_hand1(self):
        nums = [1, 2, 3, 3, 4, 5, 5, 5, 5, 5, 5, 7, 8, 9]
        s = Solution()
        result = s.is_winning_hand(nums)
        self.assertTrue(result)

    def test_is_winning_hand2(self):
        nums = [2, 3, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9]
        s = Solution()
        result = s.is_winning_hand(nums)
        self.assertFalse(result)

    def test_is_winning_hand3(self):
        nums = [1, 2, 3, 3, 4, 5, 6, 6, 7, 7, 8, 8, 9, 9]
        s = Solution()
        result = s.is_winning_hand(nums)
        self.assertTrue(result)
