#!/usr/bin/env python3

# python/tests/dsa/arrays/test_trap.py

from unittest import TestCase
from dsa.arrays.trap import Solution


class TestTrap(TestCase):
    def test_trap1(self):
        height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
        s = Solution()
        self.assertEqual(s.trap(height), 6)
        self.assertEqual(s.trap2(height), 6)
        self.assertEqual(s.trap3(height), 6)
        self.assertEqual(s.trap4(height), 6)

    def test_trap2(self):
        height = [4, 2, 0, 3, 2, 5]
        s = Solution()
        self.assertEqual(s.trap(height), 9)
        self.assertEqual(s.trap2(height), 9)
        self.assertEqual(s.trap3(height), 9)
        self.assertEqual(s.trap4(height), 9)

    def test_trap3(self):
        height = [
            6,
            4,
            2,
            0,
            3,
            2,
            0,
            3,
            1,
            4,
            5,
            3,
            2,
            7,
            5,
            3,
            0,
            1,
            2,
            1,
            3,
            4,
            6,
            8,
            1,
            3,
        ]
        s = Solution()
        self.assertEqual(s.trap(height), 83)
        self.assertEqual(s.trap2(height), 83)
        self.assertEqual(s.trap3(height), 83)
        self.assertEqual(s.trap4(height), 83)
