#!/bin/env python3

# python/tests/dsa/recursion/test_exist.py

from unittest import TestCase
from dsa.recursion.exist import Solution


class TestExist(TestCase):
    def test_exist1(self):
        board = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
        word = "ABCCED"
        s = Solution()
        self.assertTrue(s.exist(board, word))

    def test_exist2(self):
        board = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
        word = "SEE"
        s = Solution()
        self.assertTrue(s.exist(board, word))

    def test_exist3(self):
        board = [["A", "B", "C", "E"], ["S", "F", "C", "S"], ["A", "D", "E", "E"]]
        word = "ABCB"
        s = Solution()
        self.assertFalse(s.exist(board, word))

    def test_exist4(self):
        board = [["a", "b"], ["c", "d"]]
        word = "acdb"
        s = Solution()
        self.assertTrue(s.exist(board, word))
