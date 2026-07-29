#!/usr/bin/env python3

# python/tests/dsa/arrays/test_daily_temperatures.py

from unittest import TestCase
from dsa.arrays.daily_temperatures import Solution


class TestDailyTemperatures(TestCase):
    def test_daily_temperatures1(self):
        temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
        s = Solution()
        self.assertListEqual(
            s.dailyTemperatures(temperatures), [1, 1, 4, 2, 1, 1, 0, 0]
        )
        self.assertListEqual(
            s.dailyTemperatures2(temperatures), [1, 1, 4, 2, 1, 1, 0, 0]
        )

    def test_daily_temperatures2(self):
        temperatures = [30, 40, 50, 60]
        s = Solution()
        self.assertListEqual(s.dailyTemperatures(temperatures), [1, 1, 1, 0])
        self.assertListEqual(s.dailyTemperatures2(temperatures), [1, 1, 1, 0])

    def test_daily_temperatures3(self):
        temperatures = [30, 60, 90]
        s = Solution()
        self.assertListEqual(s.dailyTemperatures(temperatures), [1, 1, 0])
        self.assertListEqual(s.dailyTemperatures2(temperatures), [1, 1, 0])
