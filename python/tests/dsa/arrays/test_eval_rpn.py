#!/usr/bin/env python3

# python/tests/dsa/arrays/test_eval_rpn.py

from unittest import TestCase
from dsa.arrays.eval_rpn import Solution


class TestEvalRPN(TestCase):
    def test_eval_rpn1(self):
        tokens = ["2", "1", "+", "3", "*"]
        s = Solution()
        self.assertEqual(s.evalRPN(tokens), 9)

    def test_eval_rpn2(self):
        tokens = ["4", "13", "5", "/", "+"]
        s = Solution()
        self.assertEqual(s.evalRPN(tokens), 6)

    def test_eval_rpn3(self):
        tokens = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
        s = Solution()
        self.assertEqual(s.evalRPN(tokens), 22)
