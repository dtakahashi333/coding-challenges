#!/usr/bin/env python3

# python/tests/dsa/arrays/test_rotate.py

from unittest import TestCase
from dsa.arrays.rotate import Solution


class TestRotate(TestCase):
    def test_rotate1(self):
        matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        s = Solution()
        s.rotate(matrix)
        self.assertListEqual(matrix, [[7, 4, 1], [8, 5, 2], [9, 6, 3]])

    def test_rotate2(self):
        matrix = [[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]]
        s = Solution()
        s.rotate(matrix)
        self.assertListEqual(
            matrix, [[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]]
        )
