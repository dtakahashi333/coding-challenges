#!/bin/env python3

# python/tests/dsa/greedy_algorithms/test_can_jump.py

from unittest import TestCase
from dsa.greedy_algorithms.can_jump import Solution


class TestCanJump(TestCase):
    def test_can_jump1(self):
        nums = [2, 3, 1, 1, 4]
        s = Solution()
        self.assertTrue(s.canJump(nums))

    def test_can_jump2(self):
        nums = [3, 2, 1, 0, 4]
        s = Solution()
        self.assertFalse(s.canJump(nums))

    def test_can_jump3(self):
        nums = [0]
        s = Solution()
        self.assertTrue(s.canJump(nums))

    def test_can_jump4(self):
        nums = [0, 1]
        s = Solution()
        self.assertFalse(s.canJump(nums))

    def test_can_jump5(self):
        nums = [2, 0, 0]
        s = Solution()
        self.assertTrue(s.canJump(nums))
