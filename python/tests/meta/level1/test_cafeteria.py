#!/bin/env python3

from unittest import TestCase
from meta.level1.cafeteria import getMaxAdditionalDinersCount

class TestCafeteria(TestCase):

    def test_cafeteria1(self):
        N: int = 10
        K: int = 1
        M: int = 2
        S: list[int] = [2, 6]
        result: int = getMaxAdditionalDinersCount(N, K, M, S)
        self.assertEqual(3, result)

    def test_cafeteria2(self):
        N: int = 15
        K: int = 2
        M: int = 3
        S: list[int] = [11, 6, 14]
        result: int = getMaxAdditionalDinersCount(N, K, M, S)
        self.assertEqual(1, result)