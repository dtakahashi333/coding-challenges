#!/usr/bin/env python3

# python/tests/dsa/binary_search/test_min_eating_speed.py

from unittest import TestCase
from dsa.binary_search.min_eating_speed import Solution


class TestMinEatingSpeed(TestCase):
    def setUp(self):
        self.solution = Solution()
        return super().setUp()

    def test_min_eating_speed1(self):
        piles = [3, 6, 7, 11]
        h = 8
        self.assertEqual(self.solution.minEatingSpeed(piles, h), 4)

    def test_min_eating_speed2(self):
        piles = [30, 11, 23, 4, 20]
        h = 5
        self.assertEqual(self.solution.minEatingSpeed(piles, h), 30)

    def test_min_eating_speed3(self):
        piles = [30, 11, 23, 4, 20]
        h = 6
        self.assertEqual(self.solution.minEatingSpeed(piles, h), 23)
