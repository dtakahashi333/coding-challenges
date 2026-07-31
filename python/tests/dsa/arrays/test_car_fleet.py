#!/usr/bin/env python3

# python/tests/dsa/arrays/test_car_fleet.py

from unittest import TestCase
from dsa.arrays.car_fleet import Solution


class TestCarFleet(TestCase):
    def test_car_fleet1(self):
        target = 12
        position = [10, 8, 0, 5, 3]
        speed = [2, 4, 1, 1, 3]
        s = Solution()
        self.assertEqual(s.carFleet(target, position, speed), 3)

    def test_car_fleet2(self):
        target = 10
        position = [3]
        speed = [3]
        s = Solution()
        self.assertEqual(s.carFleet(target, position, speed), 1)

    def test_car_fleet3(self):
        target = 100
        position = [0, 2, 4]
        speed = [4, 2, 1]
        s = Solution()
        self.assertEqual(s.carFleet(target, position, speed), 1)
